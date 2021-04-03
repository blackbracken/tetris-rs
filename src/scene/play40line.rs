use std::cmp::max;
use std::collections::HashMap;
use std::time::Duration;

use ggez::{Context, GameResult, graphics};
use ggez::graphics::{DrawMode, DrawParam, PxScale, Rect};
use ggez::timer;

use crate::{FPS, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::asset::{Asset, Bgm, Se};
use crate::asset::Color as AssetColor;
use crate::input::{pressed_down, pressed_hold, pressed_move_left, pressed_move_right, pressed_pause, pressed_spin_left, pressed_spin_right, pressed_up};
use crate::router::Next;
use crate::router::Ticket::ShowTitle;
use crate::tetris::board::{DroppingMinoStatus, FIELD_UNIT_HEIGHT, FIELD_UNIT_WIDTH, FIELD_VISIBLE_UNIT_HEIGHT};
use crate::tetris::game::{DropResult, Game, Point};
use crate::tetris::tetrimino::{MinoRotation, Tetrimino};

const BLOCK_LENGTH: f32 = 32.;
const HALF_BLOCK_LENGTH: f32 = BLOCK_LENGTH / 2.;

const FONT_SIZE: f32 = 24.;

const FIELD_ORIGIN_X: f32 = WINDOW_WIDTH / 8.;
const FIELD_ORIGIN_Y: f32 = WINDOW_HEIGHT / 2. - BLOCK_LENGTH * (FIELD_VISIBLE_UNIT_HEIGHT as f32 / 2.);

const SIDE_PANEL_WIDTH: f32 = 4. * HALF_BLOCK_LENGTH;
const MINO_SPACE_IN_SIDE_PANEL_HEIGHT: f32 = HALF_BLOCK_LENGTH * 6.;

const SIDE_PANEL_PADDING: f32 = BLOCK_LENGTH;

const HOLD_ORIGIN_X: f32 = FIELD_ORIGIN_X - SIDE_PANEL_PADDING - SIDE_PANEL_WIDTH;
const HOLD_ORIGIN_Y: f32 = FIELD_ORIGIN_Y;

const NEXT_ORIGIN_X: f32 = FIELD_ORIGIN_X + (FIELD_UNIT_WIDTH as f32) * BLOCK_LENGTH + SIDE_PANEL_PADDING;
const NEXT_ORIGIN_Y: f32 = FIELD_ORIGIN_Y;

const VISIBLE_NEXT_MINO_AMOUNT: usize = 5;

pub struct Play40LineState {
    game: Game,
    ingame_elapsed: Duration,
    last_dropped: Duration,
    animation_removing: Option<RemovingLineAnimation>,
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
                animation_removing: None,
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

    // TODO: make a system to manage animations better
    let in_animating = state.countdown.is_some() || state.animation_removing.is_some();

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

    if let Some(ref mut anim) = state.animation_removing {
        anim.elapsed += diff_from_last_frame;

        if anim.is_finished() {
            state.animation_removing = None;
            state.game.remove_lines();
        }
    }

    if !in_animating {
        if pressed_pause(ctx) {
            return Ok(Next::transit(ShowTitle));
        }

        state.ingame_elapsed += diff_from_last_frame;

        update_to_hold(ctx, &mut state)?;
        update_to_move(ctx, &mut state, asset)?;

        match update_to_drop(ctx, &mut state, asset)? {
            DropEvent::Nothing => (),
            DropEvent::Dropped => {
                if let Some(lines) = state.game.board.calc_removed_lines() {
                    state.animation_removing = Some(RemovingLineAnimation::new(lines));
                }
            }
            // TODO: implement
            DropEvent::Ended => {}
        }
    }

    Ok(Next::do_continue(state.into()))
}

fn update_to_hold(
    ctx: &Context,
    state: &mut Play40LineState,
) -> GameResult {
    fn recognizes_as_hold_input(state: &mut Play40LineState, pressed: bool) -> bool {
        if pressed {
            let inputs = state.continuous_inputs
                .entry(KeyInput::Hold)
                .or_insert(0);
            *inputs += 1;
            let inputs = *inputs;

            inputs == 1
        } else {
            state.continuous_inputs.insert(KeyInput::Hold, 0);

            false
        }
    }

    let pressed_hold = pressed_hold(ctx);
    if recognizes_as_hold_input(state, pressed_hold) {
        state.game.try_swap_hold()
    }

    Ok(())
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

    let pressed_move_left = pressed_move_left(ctx);
    if recognizes_as_moving_input(state, pressed_move_left, KeyInput::MoveLeft) {
        if state.game.move_left() {
            asset.audio.play_se(ctx, Se::MinoMove)?;
        }
    }

    let pressed_move_right = pressed_move_right(ctx);
    if recognizes_as_moving_input(state, pressed_move_right, KeyInput::MoveRight) {
        if state.game.move_right() {
            asset.audio.play_se(ctx, Se::MinoMove)?;
        }
    }

    let pressed_spin_left = pressed_spin_left(ctx);
    if recognizes_as_spinning_input(state, pressed_spin_left, KeyInput::SpinLeft) {
        if state.game.spin_left() {
            asset.audio.play_se(ctx, Se::MinoSpin)?;
        }
    }

    let pressed_spin_right = pressed_spin_right(ctx);
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
) -> GameResult<DropEvent> {
    const NATURAL_DROP_INTERVAL: Duration = Duration::new(1, 0);

    fn on_put_dropping_mino(ctx: &mut Context, state: &mut Play40LineState, asset: &Asset) -> GameResult {
        state.continuous_inputs.retain(|input, _| [KeyInput::Up, KeyInput::Down].contains(input));
        state.last_dropped = state.ingame_elapsed;
        asset.audio.play_se(ctx, Se::MinoDropHardly)?;

        Ok(())
    }

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

    let pressed_up = pressed_up(ctx);
    if recognizes_as_hard_drop_input(state, pressed_up, KeyInput::Up) {
        if let Some(_) = state.game.drop_hardly() {
            on_put_dropping_mino(ctx, state, asset)?;

            return Ok(DropEvent::Dropped);
        }
    }

    let pressed_down = pressed_down(ctx);
    if recognizes_as_soft_drop_input(state, pressed_down, KeyInput::Down) {
        if state.game.board.dropping_mino_status() == DroppingMinoStatus::InAir {
            state.game.drop_softly();
            state.last_dropped = state.ingame_elapsed;
            asset.audio.play_se(ctx, Se::MinoDropSoftly)?;

            return Ok(DropEvent::Dropped);
        }
    }

    if state.last_dropped + NATURAL_DROP_INTERVAL < state.ingame_elapsed {
        let event = match state.game.drop_softly() {
            DropResult::SoftDropped => {
                asset.audio.play_se(ctx, Se::MinoDropSoftly)?;
                DropEvent::Dropped
            }
            DropResult::Put => {
                on_put_dropping_mino(ctx, state, asset)?;
                DropEvent::Dropped
            }
            DropResult::Failed => DropEvent::Ended,
        };

        state.last_dropped = state.ingame_elapsed;

        Ok(event)
    } else {
        Ok(DropEvent::Nothing)
    }
}

enum DropEvent {
    Nothing,
    Dropped,
    Ended,
}

#[derive(Hash, Eq, PartialEq)]
enum KeyInput {
    Up,
    Down,
    MoveLeft,
    MoveRight,
    SpinLeft,
    SpinRight,
    Hold,
}

pub fn draw(ctx: &mut Context, state: &Play40LineState, asset: &mut Asset) -> GameResult {
    graphics::clear(ctx, asset.color.background);

    draw_field(ctx, asset)?;
    draw_hold_panel(ctx, asset)?;
    draw_next_panel(ctx, asset)?;

    match state.countdown {
        Some(0) | None => {
            if let Some(held) = state.game.hold_mino {
                draw_hold_mino(ctx, asset, &held)?;
            }
            draw_next_minos(ctx, asset, state.game.bag.peek(VISIBLE_NEXT_MINO_AMOUNT).as_slice())?;

            if let Some(ref anim) = state.animation_removing {
                draw_minos_on_confirmed_field(ctx, asset, state, false, &anim.lines)?;
                anim.draw(ctx)?;
            } else {
                draw_minos_on_confirmed_field(ctx, asset, state, true, &Vec::new())?;
                draw_dropping_mino_prediction(ctx, state)?;
            }
        }
        Some(c) => {
            draw_count_down(ctx, asset, c)?;
        }
    }

    graphics::present(ctx)?;

    Ok(())
}

fn draw_field(ctx: &mut Context, asset: &Asset) -> GameResult {
    const FIELD_WIDTH: f32 = BLOCK_LENGTH * (FIELD_UNIT_WIDTH as f32);
    const FIELD_HEIGHT: f32 = BLOCK_LENGTH * ((FIELD_VISIBLE_UNIT_HEIGHT + 1) as f32);

    let background = graphics::Mesh::new_rectangle(
        ctx,
        DrawMode::fill(),
        Rect::new(
            FIELD_ORIGIN_X,
            FIELD_ORIGIN_Y - BLOCK_LENGTH,
            FIELD_WIDTH,
            FIELD_HEIGHT,
        ),
        asset.color.panel,
    )?;
    graphics::draw(ctx, &background, graphics::DrawParam::default())?;

    for x in (0..=FIELD_UNIT_WIDTH).map(|x| FIELD_ORIGIN_X + (x as f32) * BLOCK_LENGTH) {
        let line = graphics::Mesh::new_line(
            ctx,
            &[
                [x, FIELD_ORIGIN_Y - BLOCK_LENGTH],
                [x, FIELD_ORIGIN_Y - BLOCK_LENGTH + FIELD_HEIGHT]
            ],
            1.,
            asset.color.grid_line,
        )?;
        graphics::draw(ctx, &line, graphics::DrawParam::default())?;
    }

    for y in (-1..=(FIELD_VISIBLE_UNIT_HEIGHT as isize)).map(|y| FIELD_ORIGIN_Y + (y as f32) * BLOCK_LENGTH) {
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

    let frame = graphics::Mesh::new_rectangle(
        ctx,
        DrawMode::stroke(HALF_BLOCK_LENGTH),
        Rect::new(
            FIELD_ORIGIN_X - HALF_BLOCK_LENGTH / 2.,
            FIELD_ORIGIN_Y - HALF_BLOCK_LENGTH * 3. / 2.,
            FIELD_WIDTH + HALF_BLOCK_LENGTH - 1.0,
            FIELD_HEIGHT - 1.0,
        ),
        asset.color.frame,
    )?;
    graphics::draw(ctx, &frame, graphics::DrawParam::default())?;

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

fn draw_minos_on_confirmed_field(
    ctx: &mut Context,
    asset: &mut Asset,
    state: &Play40LineState,
    shows_dropping_mino: bool,
    hidden_lines: &Vec<usize>,
) -> GameResult {
    let board = state.game.board;
    let field = if shows_dropping_mino {
        board.field()
    } else {
        board.confirmed_field
    };

    // 0-20th lines
    for y in (0..(FIELD_VISIBLE_UNIT_HEIGHT)).into_iter().filter(|&n| !hidden_lines.contains(&(n + 1))) {
        for x in 0..FIELD_UNIT_WIDTH {
            let entity = field
                .get(y + (FIELD_UNIT_HEIGHT - FIELD_VISIBLE_UNIT_HEIGHT))
                .and_then(|array| array.get(x))
                .unwrap();

            if let Some(block) = entity.block() {
                let img = asset.image.mino_block(ctx, &block)?;
                let x = (FIELD_ORIGIN_X as f32) + (x as f32) * BLOCK_LENGTH;
                let y = (FIELD_ORIGIN_Y as f32) + (y as f32) * BLOCK_LENGTH;

                graphics::draw(
                    ctx,
                    img,
                    graphics::DrawParam::default().dest([x, y]),
                )?;
            }
        }
    }

    // 21st line
    let line = field
        .get(FIELD_UNIT_HEIGHT - FIELD_VISIBLE_UNIT_HEIGHT - 1)
        .unwrap();
    for (x, e) in line.iter().enumerate() {
        if let Some(block) = e.block() {
            let img = asset.image.mino_block(ctx, &block)?;
            let x = (FIELD_ORIGIN_X as f32) + (x as f32) * BLOCK_LENGTH;
            let y = (FIELD_ORIGIN_Y as f32) - BLOCK_LENGTH / 2.;

            graphics::draw(
                ctx,
                img,
                graphics::DrawParam::default()
                    .src(graphics::Rect::new(0., 0.5, 1., 0.5))
                    .dest([x, y]),
            )?;
        }
    }

    Ok(())
}

fn draw_dropping_mino_prediction(ctx: &mut Context, state: &Play40LineState) -> GameResult {
    const PREDICTION_PADDING: f32 = 3.;

    let field = state.game.board.field();
    let color = AssetColor::block(&state.game.board.dropping.block());
    let color = graphics::Color::from([color.r, color.g, color.b, 0.85]);

    for prediction in state.game.board.calc_dropping_mino_prediction() {
        let entity = field
            .get(prediction.y as usize)
            .and_then(|line| line.get(prediction.x as usize))
            .unwrap();

        if entity.is_air() {
            let x = (FIELD_ORIGIN_X as f32) + (prediction.x as f32) * BLOCK_LENGTH + PREDICTION_PADDING;
            let y = (FIELD_ORIGIN_Y as f32) + ((prediction.y - (FIELD_UNIT_HEIGHT - FIELD_VISIBLE_UNIT_HEIGHT) as isize) as f32) * BLOCK_LENGTH + PREDICTION_PADDING;


            let square = graphics::Mesh::new_rectangle(
                ctx,
                DrawMode::stroke(2.),
                Rect::new(
                    x,
                    y,
                    BLOCK_LENGTH - 2. * PREDICTION_PADDING,
                    BLOCK_LENGTH - 2. * PREDICTION_PADDING,
                ),
                color,
            )?;

            graphics::draw(
                ctx,
                &square,
                DrawParam::default(),
            )?;
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

fn draw_hold_mino(ctx: &mut Context, asset: &mut Asset, mino: &Tetrimino) -> GameResult {
    let x = HOLD_ORIGIN_X + match mino {
        Tetrimino::O => HALF_BLOCK_LENGTH,
        _ => 0.,
    };
    let y = HOLD_ORIGIN_Y + 2. * HALF_BLOCK_LENGTH + match mino {
        Tetrimino::I => -HALF_BLOCK_LENGTH,
        _ => 0.,
    };

    draw_mini_mino(
        ctx,
        asset,
        mino,
        (x, y).into(),
    )?;

    Ok(())
}

fn draw_next_panel(ctx: &mut Context, asset: &Asset) -> GameResult {
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

    for idx in 1..=VISIBLE_NEXT_MINO_AMOUNT {
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

struct RemovingLineAnimation {
    lines: Vec<usize>,
    elapsed: Duration,
}

const PHASE_1: Duration = Duration::from_secs_f32(0.3);
const PHASE_2: Duration = Duration::from_secs_f32(0.75);

impl RemovingLineAnimation {
    fn new(lines: Vec<usize>) -> RemovingLineAnimation {
        RemovingLineAnimation {
            lines,
            elapsed: Duration::ZERO,
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        let elapsed = self.elapsed;

        if elapsed < PHASE_1 {
            let alpha = 1. - (PHASE_1 - elapsed).as_secs_f32() / PHASE_1.as_secs_f32();
            let color = graphics::Color::new(1., 1., 1., alpha);

            for line in &self.lines {
                let rect = graphics::Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect::new(
                        FIELD_ORIGIN_X,
                        FIELD_ORIGIN_Y + (*line as f32 - 1.) * BLOCK_LENGTH,
                        (FIELD_UNIT_WIDTH as f32) * BLOCK_LENGTH,
                        BLOCK_LENGTH,
                    ),
                    color,
                )?;

                graphics::draw(ctx, &rect, DrawParam::default())?;
            }
        } else if elapsed < PHASE_2 {
            let color = graphics::Color::from_rgb(255, 255, 255);

            for line in &self.lines {
                let rect = graphics::Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect::new(
                        FIELD_ORIGIN_X,
                        FIELD_ORIGIN_Y + (*line as f32 - 1.) * BLOCK_LENGTH,
                        (FIELD_UNIT_WIDTH as f32) * BLOCK_LENGTH,
                        BLOCK_LENGTH,
                    ),
                    color,
                )?;

                graphics::draw(ctx, &rect, DrawParam::default())?;
            }
        }

        Ok(())
    }

    fn is_finished(&self) -> bool {
        self.elapsed > PHASE_2
    }
}