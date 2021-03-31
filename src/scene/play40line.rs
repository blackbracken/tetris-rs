use std::cmp::max;
use std::collections::HashMap;
use std::time::Duration;

use ggez::{Context, GameResult, graphics};
use ggez::event::KeyCode;
use ggez::graphics::{DrawMode, PxScale, Rect};
use ggez::input::gamepad::gamepads;
use ggez::input::gamepad::gilrs::Button;
use ggez::input::keyboard;
use ggez::timer;

use crate::{FPS, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::asset::{Asset, Bgm, Se};
use crate::router::Next;
use crate::router::SceneState::ForPlay40Line;
use crate::router::Ticket::ShowTitle;
use crate::tetris::game::{DroppingMinoStatus, DropResult, FIELD_UNIT_HEIGHT, FIELD_UNIT_WIDTH, FIELD_VISIBLE_UNIT_HEIGHT, Game, Point};
use crate::tetris::tetrimino::{MinoRotation, Tetrimino};

const BLOCK_LENGTH: f32 = 32.;
const HALF_BLOCK_LENGTH: f32 = BLOCK_LENGTH / 2.;

const FONT_SIZE: f32 = 24.;

const FIELD_ORIGIN_X: f32 = WINDOW_WIDTH / 8.;
const FIELD_ORIGIN_Y: f32 = WINDOW_HEIGHT / 2. - BLOCK_LENGTH * (FIELD_VISIBLE_UNIT_HEIGHT as f32 / 2.);

const SIDE_PANEL_WIDTH: f32 = 4. * HALF_BLOCK_LENGTH;
const MINO_SPACE_IN_SIDE_PANEL_HEIGHT: f32 = HALF_BLOCK_LENGTH * 6.;

const HOLD_ORIGIN_X: f32 = FIELD_ORIGIN_X - 16. - SIDE_PANEL_WIDTH;
const HOLD_ORIGIN_Y: f32 = FIELD_ORIGIN_Y;

const NEXT_ORIGIN_X: f32 = FIELD_ORIGIN_X + (FIELD_UNIT_WIDTH as f32) * BLOCK_LENGTH + 16.;
const NEXT_ORIGIN_Y: f32 = FIELD_ORIGIN_Y;

pub struct Play40LineState {
    game: Game,
    ingame_elapsed: Duration,
    // TODO: generalize (e.g. scheduler)
    last_dropped: Duration,
    countdown: Option<u64>,
    start_countdown_at: Duration,
    continuous_inputs: HashMap<KeyInput, usize>,
}

impl Play40LineState {
    pub fn new(ctx: &mut Context) -> GameResult<Play40LineState> {
        Ok(
            Play40LineState {
                game: Game::new(),
                ingame_elapsed: Duration::ZERO,
                last_dropped: Duration::ZERO,
                countdown: Some(3),
                start_countdown_at: timer::time_since_start(ctx),
                continuous_inputs: HashMap::new(),
            }
        )
    }
}

pub fn init(_ctx: &mut Context, asset: &mut Asset) {
    asset.audio.stop_bgm();
}

pub fn update(
    ctx: &mut Context,
    mut state: Play40LineState,
    asset: &mut Asset,
    diff_from_last_frame: Duration,
) -> GameResult<Next> {
    const COUNTDOWN_SEC: u64 = 3;

    let countdown = match state.countdown {
        None | Some(0) => None,
        Some(_) => {
            let diff = timer::time_since_start(ctx)
                .saturating_sub(state.start_countdown_at)
                .as_secs();
            Some(max(0, COUNTDOWN_SEC - diff))
        }
    };

    if state.countdown != countdown {
        match countdown {
            Some(0) => {
                asset.audio.play_bgm(ctx, Bgm::InGame)?;
                asset.audio.play_se(ctx, Se::GameStart)?;
            }
            Some(_) => {
                asset.audio.play_se(ctx, Se::CountdownTick)?;
            }
            None => (),
        }

        state.countdown = countdown;
    }

    if state.countdown == None {
        if keyboard::is_key_pressed(ctx, KeyCode::Escape) {
            return Ok(Next::transit(ShowTitle));
        }

        state.ingame_elapsed += diff_from_last_frame;

        update_to_move(ctx, &mut state, asset)?;

        // TODO: implement
        match update_to_drop(ctx, &mut state, asset)? {
            TetrisResult::Continue => {}
            TetrisResult::End => {}
        }
    }

    Ok(Next::do_continue(state.into()))
}

fn update_to_move(
    ctx: &mut Context,
    state: &mut Play40LineState,
    asset: &mut Asset,
) -> GameResult {
    fn recognizes_as_moving_input(state: &mut Play40LineState, pressed: bool, key_input: KeyInput) -> bool {
        const CONTINUOUS_WAIT: usize = (FPS as usize) * 2 / 5;
        const CONTINUOUS_INTERVAL: usize = (FPS as usize) / 20;

        if pressed {
            let inputs = state.continuous_inputs
                .entry(key_input)
                .or_insert(0);
            *inputs += 1;
            let inputs = *inputs;

            inputs == 1 || (inputs >= CONTINUOUS_WAIT && inputs % CONTINUOUS_INTERVAL == 0)
        } else {
            state.continuous_inputs.insert(key_input, 0);

            false
        }
    }

    fn recognizes_as_spinning_input(state: &mut Play40LineState, pressed: bool, key_input: KeyInput) -> bool {
        if pressed {
            let inputs = state.continuous_inputs
                .entry(key_input)
                .or_insert(0);
            *inputs += 1;
            let inputs = *inputs;

            inputs == 1
        } else {
            state.continuous_inputs.insert(key_input, 0);

            false
        }
    }

    let pressed_move_left = [KeyCode::A, KeyCode::Left].iter()
        .any(|&key| keyboard::is_key_pressed(ctx, key))
        || gamepads(ctx).any(|(_, pad)| pad.is_pressed(Button::DPadLeft));
    if recognizes_as_moving_input(state, pressed_move_left, KeyInput::MoveLeft) {
        if state.game.move_left() {
            asset.audio.play_se(ctx, Se::MinoMove)?;
        }
    }

    let pressed_move_right = [KeyCode::D, KeyCode::Right].iter()
        .any(|&key| keyboard::is_key_pressed(ctx, key))
        || gamepads(ctx).any(|(_, pad)| pad.is_pressed(Button::DPadRight));
    if recognizes_as_moving_input(state, pressed_move_right, KeyInput::MoveRight) {
        if state.game.move_right() {
            asset.audio.play_se(ctx, Se::MinoMove)?;
        }
    }

    let pressed_spin_left = keyboard::is_key_pressed(ctx, KeyCode::J)
        || gamepads(ctx).any(|(_, pad)| pad.is_pressed(Button::South));
    if recognizes_as_spinning_input(state, pressed_spin_left, KeyInput::SpinLeft) {
        if state.game.spin_left() {
            asset.audio.play_se(ctx, Se::MinoSpin)?;
        }
    }

    let pressed_spin_right = keyboard::is_key_pressed(ctx, KeyCode::K)
        || gamepads(ctx).any(|(_, pad)| pad.is_pressed(Button::East));
    if recognizes_as_spinning_input(state, pressed_spin_right, KeyInput::SpinRight) {
        if state.game.spin_right() {
            asset.audio.play_se(ctx, Se::MinoSpin)?;
        }
    }

    Ok(())
}

fn update_to_drop(
    ctx: &mut Context,
    state: &mut Play40LineState,
    asset: &mut Asset,
) -> GameResult<TetrisResult> {
    const NATURAL_DROP_INTERVAL: Duration = Duration::new(1, 0);

    fn recognizes_as_hard_drop_input(state: &mut Play40LineState, pressed: bool, key_input: KeyInput) -> bool {
        const CONTINUOUS_WAIT: usize = (FPS as usize) * 2 / 5;
        const CONTINUOUS_INTERVAL: usize = (FPS as usize) / 4;

        if pressed {
            let inputs = state.continuous_inputs
                .entry(key_input)
                .or_insert(0);
            *inputs += 1;
            let inputs = *inputs;

            inputs == 1 || (inputs >= CONTINUOUS_WAIT && inputs % CONTINUOUS_INTERVAL == 0)
        } else {
            state.continuous_inputs.insert(key_input, 0);

            false
        }
    }

    fn recognizes_as_soft_drop_input(state: &mut Play40LineState, pressed: bool, key_input: KeyInput) -> bool {
        const CONTINUOUS_WAIT: usize = (FPS as usize) * 2 / 5;
        const CONTINUOUS_INTERVAL: usize = (FPS as usize) / 20;

        if pressed {
            let inputs = state.continuous_inputs
                .entry(key_input)
                .or_insert(0);
            *inputs += 1;
            let inputs = *inputs;

            inputs == 1 || (inputs >= CONTINUOUS_WAIT && inputs % CONTINUOUS_INTERVAL == 0)
        } else {
            state.continuous_inputs.insert(key_input, 0);

            false
        }
    }

    let pressed_up = keyboard::is_key_pressed(ctx, KeyCode::W)
        || gamepads(ctx).any(|(_, pad)| pad.is_pressed(Button::DPadUp));
    if recognizes_as_hard_drop_input(state, pressed_up, KeyInput::Up) {
        if let Some(_) = state.game.drop_hardly() {
            on_put_dropping_mino(ctx, state, asset)?;
        }
    }

    let pressed_down = keyboard::is_key_pressed(ctx, KeyCode::S)
        || gamepads(ctx).any(|(_, pad)| pad.is_pressed(Button::DPadDown));
    if recognizes_as_soft_drop_input(state, pressed_down, KeyInput::Down) {
        if state.game.board.dropping_mino_status() == DroppingMinoStatus::InAir {
            state.game.drop_softly();
            state.last_dropped = state.ingame_elapsed;
            asset.audio.play_se(ctx, Se::MinoDropSoftly);
        }
    }

    if state.last_dropped + NATURAL_DROP_INTERVAL < state.ingame_elapsed {
        match state.game.drop_softly() {
            DropResult::SoftDropped => {
                asset.audio.play_se(ctx, Se::MinoDropSoftly);
            }
            DropResult::Put => {
                on_put_dropping_mino(ctx, state, asset)?;
            }
            DropResult::Failed => {
                return Ok(TetrisResult::End);
            }
        }

        state.last_dropped = state.ingame_elapsed;
    }

    Ok(TetrisResult::Continue)
}

fn on_put_dropping_mino(ctx: &mut Context, state: &mut Play40LineState, asset: &Asset) -> GameResult {
    state.continuous_inputs.retain(|input, _| [KeyInput::Up, KeyInput::Down].contains(input));
    asset.audio.play_se(ctx, Se::MinoDropHardly)?;

    Ok(())
}

enum TetrisResult {
    Continue,
    End, // Success | Fail
}

#[derive(Hash, Eq, PartialEq)]
enum KeyInput {
    Up,
    Down,
    MoveLeft,
    MoveRight,
    SpinLeft,
    SpinRight,
}

pub fn draw(ctx: &mut Context, state: &Play40LineState, asset: &mut Asset) -> GameResult {
    graphics::clear(ctx, asset.color.background);

    draw_field_grid(ctx, asset)?;
    draw_hold_panel(ctx, asset)?;
    draw_next_panel(ctx, asset)?;

    match state.countdown {
        Some(0) | None => {
            draw_next_minos(ctx, asset, state.game.bag.peek(5).as_slice())?;
            draw_minos_on_field(ctx, asset, state)?;
        }
        Some(c) => {
            draw_count_down(ctx, asset, c)?;
        }
    }

    graphics::present(ctx)?;

    Ok(())
}

fn draw_field_grid(ctx: &mut Context, asset: &Asset) -> GameResult {
    const FIELD_WIDTH: f32 = BLOCK_LENGTH * (FIELD_UNIT_WIDTH as f32);
    const FIELD_HEIGHT: f32 = BLOCK_LENGTH * (FIELD_VISIBLE_UNIT_HEIGHT as f32);

    let rect = graphics::Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        Rect::new(
            FIELD_ORIGIN_X,
            FIELD_ORIGIN_Y,
            FIELD_WIDTH,
            FIELD_HEIGHT,
        ),
        asset.color.panel,
    )?;
    graphics::draw(ctx, &rect, graphics::DrawParam::default())?;

    for x in (0..=FIELD_UNIT_WIDTH).map(|x| FIELD_ORIGIN_X + (x as f32) * BLOCK_LENGTH) {
        let line = graphics::Mesh::new_line(
            ctx,
            &[
                [x, FIELD_ORIGIN_Y],
                [x, FIELD_ORIGIN_Y + FIELD_HEIGHT]
            ],
            1.,
            asset.color.grid_line,
        )?;
        graphics::draw(ctx, &line, graphics::DrawParam::default())?;
    }

    for y in (0..=FIELD_VISIBLE_UNIT_HEIGHT).map(|y| FIELD_ORIGIN_Y + (y as f32) * BLOCK_LENGTH) {
        let line = graphics::Mesh::new_line(
            ctx,
            &[
                [FIELD_ORIGIN_X, y],
                [FIELD_ORIGIN_X + FIELD_WIDTH, y]
            ],
            1.,
            asset.color.grid_line,
        )?;
        graphics::draw(ctx, &line, graphics::DrawParam::default())?;
    }

    Ok(())
}

fn draw_count_down(ctx: &mut Context, asset: &Asset, sec: u64) -> GameResult {
    let rect = graphics::Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        Rect::new(0., 0., WINDOW_WIDTH, WINDOW_HEIGHT),
        graphics::Color::new(0., 0., 0., 0.9),
    )?;

    graphics::draw(ctx, &rect, graphics::DrawParam::default())?;

    let text = graphics::Text::new(
        graphics::TextFragment::new(sec.to_string())
            .font(asset.font.vt323)
            .scale(PxScale::from(200.))
    );

    graphics::draw(
        ctx,
        &text,
        graphics::DrawParam::default()
            .dest([
                WINDOW_WIDTH / 2. - text.width(ctx) / 2.,
                WINDOW_HEIGHT / 2. - text.height(ctx) / 2.,
            ]),
    )?;

    Ok(())
}

fn draw_minos_on_field(ctx: &mut Context, asset: &mut Asset, state: &Play40LineState) -> GameResult {
    let field = &state.game.board.field();
    for y in 0..FIELD_VISIBLE_UNIT_HEIGHT {
        for x in 0..FIELD_UNIT_WIDTH {
            let entity = field
                .get(y + (FIELD_UNIT_HEIGHT - FIELD_VISIBLE_UNIT_HEIGHT))
                .and_then(|array| array.get(x))
                .unwrap();

            if let Some(block) = entity.block() {
                let img = asset.image.mino_block(ctx, &block)?;
                let x = x as f32;
                let y = y as f32;

                graphics::draw(
                    ctx,
                    img,
                    graphics::DrawParam::default()
                        .dest([
                            (FIELD_ORIGIN_X as f32) + (x * BLOCK_LENGTH) as f32,
                            (FIELD_ORIGIN_Y as f32) + (y * BLOCK_LENGTH) as f32,
                        ]),
                )?;
            }
        }
    }

    Ok(())
}

fn draw_hold_panel(ctx: &mut Context, asset: &Asset) -> GameResult {
    let text = graphics::Text::new(
        graphics::TextFragment::new("HOLD")
            .font(asset.font.vt323)
            .scale(PxScale::from(FONT_SIZE))
    );
    graphics::draw(
        ctx,
        &text,
        graphics::DrawParam::default()
            .dest([HOLD_ORIGIN_X, HOLD_ORIGIN_Y - FONT_SIZE]),
    )?;

    let line = graphics::Mesh::new_line(
        ctx,
        &[
            [HOLD_ORIGIN_X, HOLD_ORIGIN_Y],
            [HOLD_ORIGIN_X + SIDE_PANEL_WIDTH, HOLD_ORIGIN_Y]
        ],
        2.,
        asset.color.separator,
    )?;
    graphics::draw(ctx, &line, graphics::DrawParam::default())?;

    let line = graphics::Mesh::new_line(
        ctx,
        &[
            [HOLD_ORIGIN_X, HOLD_ORIGIN_Y + MINO_SPACE_IN_SIDE_PANEL_HEIGHT],
            [HOLD_ORIGIN_X + SIDE_PANEL_WIDTH, HOLD_ORIGIN_Y + MINO_SPACE_IN_SIDE_PANEL_HEIGHT]
        ],
        1.,
        asset.color.separator,
    )?;
    graphics::draw(ctx, &line, graphics::DrawParam::default())?;

    Ok(())
}

fn draw_next_panel(ctx: &mut Context, asset: &Asset) -> GameResult {
    const NEXT_MINOS_AMOUNT: usize = 5;

    let text = graphics::Text::new(
        graphics::TextFragment::new("NEXT")
            .font(asset.font.vt323)
            .scale(PxScale::from(FONT_SIZE))
    );
    graphics::draw(
        ctx,
        &text,
        graphics::DrawParam::default()
            .dest([NEXT_ORIGIN_X, NEXT_ORIGIN_Y - FONT_SIZE]),
    )?;

    let line = graphics::Mesh::new_line(
        ctx,
        &[
            [NEXT_ORIGIN_X, NEXT_ORIGIN_Y],
            [NEXT_ORIGIN_X + SIDE_PANEL_WIDTH, NEXT_ORIGIN_Y]
        ],
        2.,
        asset.color.separator,
    )?;
    graphics::draw(ctx, &line, graphics::DrawParam::default())?;

    for idx in 1..=NEXT_MINOS_AMOUNT {
        let y = NEXT_ORIGIN_Y + (idx as f32) * MINO_SPACE_IN_SIDE_PANEL_HEIGHT;
        let sep = graphics::Mesh::new_line(
            ctx,
            &[
                [NEXT_ORIGIN_X, y],
                [NEXT_ORIGIN_X + SIDE_PANEL_WIDTH, y]
            ],
            1.,
            asset.color.separator,
        )?;

        graphics::draw(ctx, &sep, graphics::DrawParam::default())?;
    }

    Ok(())
}

fn draw_next_minos(ctx: &mut Context, asset: &mut Asset, minos: &[Tetrimino]) -> GameResult {
    for (idx, mino) in minos.iter().enumerate() {
        let x = NEXT_ORIGIN_X + match mino {
            Tetrimino::O => HALF_BLOCK_LENGTH,
            _ => 0.,
        };
        let y = NEXT_ORIGIN_Y
            + (idx as f32) * MINO_SPACE_IN_SIDE_PANEL_HEIGHT
            + 2. * HALF_BLOCK_LENGTH
            + match mino {
            Tetrimino::I => -HALF_BLOCK_LENGTH,
            _ => 0.,
        };

        draw_mini_mino(
            ctx,
            asset,
            mino,
            (x, y).into(),
        )?;
    }

    Ok(())
}

fn draw_mini_mino(ctx: &mut Context, asset: &mut Asset, mino: &Tetrimino, point: Point) -> GameResult {
    let shapes = mino.shapes();
    let shape = shapes.get(&MinoRotation::Clockwise).unwrap();

    for (y, line) in shape.iter().enumerate() {
        for (x, &exists) in line.iter().enumerate() {
            let x = (point.x as f32) + HALF_BLOCK_LENGTH * (x as f32);
            let y = (point.y as f32) + HALF_BLOCK_LENGTH * (y as f32);

            if exists {
                let img = asset.image.mino_block(ctx, &mino.block())?;
                graphics::draw(
                    ctx,
                    img,
                    graphics::DrawParam::default()
                        .dest([x, y])
                        .scale([0.5, 0.5]),
                )?;
            }
        }
    }

    Ok(())
}