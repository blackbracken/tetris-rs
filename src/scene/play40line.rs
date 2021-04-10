use std::cmp::max;
use std::collections::HashMap;
use std::time::Duration;

use ggez::{Context, GameResult, graphics};
use ggez::graphics::{DrawMode, DrawParam, PxScale, Rect};
use ggez::timer;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::asset::{Asset, Bgm, Se};
use crate::asset::Color as AssetColor;
use crate::input::{pressed_down, pressed_hold, pressed_move_left, pressed_move_right, pressed_pause, pressed_spin_left, pressed_spin_right, pressed_up};
use crate::router::Next;
use crate::router::Ticket::ShowTitle;
use crate::tetris::board::{FIELD_UNIT_HEIGHT, FIELD_UNIT_WIDTH, FIELD_VISIBLE_UNIT_HEIGHT};
use crate::tetris::game::{DroppedOrNothing, Game, Point, PutOrJustDropped};
use crate::tetris::tetrimino::{MinoRotation, Tetrimino};

const BLOCK_LENGTH: f32 = 32.;
const HALF_BLOCK_LENGTH: f32 = BLOCK_LENGTH / 2.;

const PANEL_FONT_SIZE: f32 = 24.;

const FIELD_ORIGIN_X: f32 = WINDOW_WIDTH / 8.;
const FIELD_ORIGIN_Y: f32 = WINDOW_HEIGHT / 2. - BLOCK_LENGTH * (FIELD_VISIBLE_UNIT_HEIGHT as f32 / 2.);

const FIELD_HEIGHT: f32 = (FIELD_UNIT_HEIGHT as f32) * BLOCK_LENGTH;

const SIDE_PANEL_WIDTH: f32 = 4. * HALF_BLOCK_LENGTH;
const MINO_SPACE_IN_SIDE_PANEL_HEIGHT: f32 = HALF_BLOCK_LENGTH * 6.;

const SIDE_PANEL_PADDING: f32 = BLOCK_LENGTH;

const HOLD_ORIGIN_X: f32 = FIELD_ORIGIN_X - SIDE_PANEL_PADDING - SIDE_PANEL_WIDTH;
const HOLD_ORIGIN_Y: f32 = FIELD_ORIGIN_Y;

const NEXT_ORIGIN_X: f32 = FIELD_ORIGIN_X + (FIELD_UNIT_WIDTH as f32) * BLOCK_LENGTH + SIDE_PANEL_PADDING;
const NEXT_ORIGIN_Y: f32 = FIELD_ORIGIN_Y;

const TEXTS_FONT_SIZE: f32 = 42.;
const TEXTS_Y_MARGIN: f32 = FIELD_HEIGHT / 12.;
const TEXTS_PADDING: f32 = 1.5 * TEXTS_FONT_SIZE;

const TEXTS_ORIGIN_X: f32 = NEXT_ORIGIN_X + 2. * SIDE_PANEL_WIDTH;
const TEXTS_ORIGIN_Y: f32 = FIELD_ORIGIN_Y + TEXTS_Y_MARGIN;

const VISIBLE_NEXT_MINO_AMOUNT: usize = 5;

pub struct Play40LineState {
    game: Game,
    ingame_elapsed: Duration,
    animation_removing: Option<RemovingLineAnimation>,
    countdown: Option<u64>,
    start_countdown_at: Duration,
    continuous_input: ContinuousInput,
}

impl Play40LineState {
    pub fn new(ctx: &mut Context) -> GameResult<Play40LineState> {
        Ok(
            Play40LineState {
                game: Game::new(),
                ingame_elapsed: Duration::ZERO,
                animation_removing: None,
                countdown: Some(3),
                start_countdown_at: timer::time_since_start(ctx),
                continuous_input: ContinuousInput::new(),
            }
        )
    }
}

struct ContinuousInput {
    elapsed: Duration,
    elapsed_from_continuous_first_input: HashMap<KeyInput, Duration>,
    elapsed_from_continuous_last_input: HashMap<KeyInput, Duration>,

    continuity_counter: u64,
    last_input_at: HashMap<KeyInput, u64>,
}

impl ContinuousInput {
    fn new() -> ContinuousInput {
        ContinuousInput {
            elapsed: Duration::ZERO,
            elapsed_from_continuous_first_input: HashMap::new(),
            elapsed_from_continuous_last_input: HashMap::new(),
            last_input_at: HashMap::new(),
            continuity_counter: 0,
        }
    }

    fn inputted_just_before(&self, key: &KeyInput) -> bool {
        self.last_input_at
            .get(key)
            .filter(|c| *c + 1 == self.continuity_counter)
            .is_none()
    }

    pub fn elapse(&mut self, delta: Duration) {
        self.elapsed += delta;
        self.continuity_counter += 1;

        let keys = self.elapsed_from_continuous_first_input
            .iter()
            .filter(|(k, _)| !self.inputted_just_before(k))
            .map(|(k, _)| k.to_owned())
            .collect::<Vec<_>>();
        self.elapsed_from_continuous_first_input.retain(|key, _| keys.contains(key));

        let keys = self.elapsed_from_continuous_last_input
            .iter()
            .filter(|(k, _)| !self.inputted_just_before(k))
            .map(|(k, _)| k.to_owned())
            .collect::<Vec<_>>();
        self.elapsed_from_continuous_last_input.retain(|key, _| keys.contains(key));
    }

    pub fn input(&mut self, key: KeyInput) -> bool {
        let valid = self.is_input_valid(&key);

        self.last_input_at.insert(key, self.continuity_counter);
        if valid {
            self.elapsed_from_continuous_first_input.entry(key).or_insert(self.elapsed);
            self.elapsed_from_continuous_last_input.insert(key, self.elapsed);
        }

        valid
    }

    fn is_input_valid(&self, key: &KeyInput) -> bool {
        let inputted_just_before = self.inputted_just_before(key);
        let first = self.elapsed_from_continuous_first_input.get(key)
            .unwrap_or(&Duration::ZERO)
            .to_owned();
        let last = self.elapsed_from_continuous_last_input.get(key)
            .unwrap_or(&Duration::ZERO)
            .to_owned();

        match key {
            KeyInput::Up => {
                inputted_just_before
                    || (first + Duration::from_secs_f32(0.4) < self.elapsed && last + Duration::from_secs_f32(0.25) < self.elapsed)
            }
            KeyInput::Down => {
                inputted_just_before
                    || (first + Duration::from_secs_f32(0.4) < self.elapsed && last + Duration::from_secs_f32(0.03) < self.elapsed)
            }
            KeyInput::MoveLeft => {
                inputted_just_before
                    || (first + Duration::from_secs_f32(0.4) < self.elapsed && last + Duration::from_secs_f32(0.03) < self.elapsed)
            }
            KeyInput::MoveRight => {
                inputted_just_before
                    || (first + Duration::from_secs_f32(0.4) < self.elapsed && last + Duration::from_secs_f32(0.03) < self.elapsed)
            }
            KeyInput::SpinLeft => {
                inputted_just_before
            }
            KeyInput::SpinRight => {
                inputted_just_before
            }
            KeyInput::Hold => {
                inputted_just_before
            }
        }
    }
}


pub fn init(_ctx: &mut Context, asset: &mut Asset) {
    asset.audio.stop_bgm();
}

pub fn update(
    ctx: &mut Context,
    mut state: Play40LineState,
    asset: &mut Asset,
    delta: Duration,
) -> GameResult<Next> {
    const COUNTDOWN_SEC: u64 = 3;

    // TODO: make a system to manage animations better
    let in_animating = state.countdown.is_some() || state.animation_removing.is_some();
    state.continuous_input.elapse(delta);

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
        anim.elapsed += delta;

        if anim.is_finished() {
            state.animation_removing = None;
            state.game.remove_lines();
        }
    }

    if !in_animating {
        if pressed_pause(ctx) {
            return Ok(Next::transit(ShowTitle));
        }

        state.ingame_elapsed += delta;

        update_to_hold(ctx, &mut state)?;
        update_to_move(ctx, &mut state, asset)?;

        if let DroppedOrNothing::Dropped(put_or_just_dropped) = update_to_drop(ctx, &mut state)? {
            on_drop(ctx, &mut state, asset, put_or_just_dropped)?;
        } else if let DroppedOrNothing::Dropped(put_or_just_dropped) = state.game.elapse(delta) {
            on_drop(ctx, &mut state, asset, put_or_just_dropped)?;
        }
    }

    Ok(Next::do_continue(state.into()))
}

fn on_drop(ctx: &mut Context, state: &mut Play40LineState, asset: &Asset, put_or_dropped: PutOrJustDropped) -> GameResult {
    if let Some(removed_lines) = put_or_dropped {
        asset.audio.play_se(ctx, Se::MinoHardDrop)?;

        if !removed_lines.is_empty() {
            asset.audio.play_se(ctx, Se::RemoveLine)?;
            state.animation_removing = Some(RemovingLineAnimation::new(removed_lines));
        }

        if !state.game.put_and_spawn() {
            unimplemented!("the game is over")
        }
    } else {
        asset.audio.play_se(ctx, Se::MinoSoftDrop)?;
    }

    Ok(())
}

fn update_to_hold(
    ctx: &Context,
    state: &mut Play40LineState,
) -> GameResult {
    if pressed_hold(ctx) && state.continuous_input.input(KeyInput::Hold) {
        state.game.try_swap_hold()
    }

    Ok(())
}

fn update_to_move(
    ctx: &mut Context,
    state: &mut Play40LineState,
    asset: &mut Asset,
) -> GameResult {
    if pressed_move_left(ctx) && state.continuous_input.input(KeyInput::MoveLeft) {
        if state.game.move_left() {
            asset.audio.play_se(ctx, Se::MinoMove)?;
        }
    }

    if pressed_move_right(ctx) && state.continuous_input.input(KeyInput::MoveRight) {
        if state.game.move_right() {
            asset.audio.play_se(ctx, Se::MinoMove)?;
        }
    }

    if pressed_spin_left(ctx) && state.continuous_input.input(KeyInput::SpinLeft) {
        if state.game.spin_left() {
            asset.audio.play_se(ctx, Se::MinoSpin)?;
        }
    }

    if pressed_spin_right(ctx) && state.continuous_input.input(KeyInput::SpinRight) {
        if state.game.spin_right() {
            asset.audio.play_se(ctx, Se::MinoSpin)?;
        }
    }

    Ok(())
}

fn update_to_drop(
    ctx: &mut Context,
    state: &mut Play40LineState,
) -> GameResult<DroppedOrNothing> {
    if pressed_up(ctx) && state.continuous_input.input(KeyInput::Up) {
        return Ok(
            DroppedOrNothing::dropped(Some(state.game.hard_drop()))
        );
    }

    if pressed_down(ctx) && state.continuous_input.input(KeyInput::Down) {
        if !state.game.board.dropping_mino_is_on_ground() {
            return Ok(
                DroppedOrNothing::dropped(state.game.soft_drop())
            );
        }
    }

    Ok(DroppedOrNothing::Nothing)
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
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

    draw_total_score(ctx, asset, state.game.score)?;
    draw_removed_line_count(ctx, asset, state.game.removed_line_count)?;
    draw_timer(ctx, asset, &state.game.elapsed)?;

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
            .scale(PxScale::from(PANEL_FONT_SIZE))
    );
    graphics::draw(
        ctx,
        &text,
        graphics::DrawParam::default()
            .dest([HOLD_ORIGIN_X, HOLD_ORIGIN_Y - PANEL_FONT_SIZE]),
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
            .scale(PxScale::from(PANEL_FONT_SIZE))
    );
    graphics::draw(
        ctx,
        &text,
        graphics::DrawParam::default()
            .dest([NEXT_ORIGIN_X, NEXT_ORIGIN_Y - PANEL_FONT_SIZE]),
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

fn draw_total_score(ctx: &mut Context, asset: &Asset, score: usize) -> GameResult {
    let text = format!("{0: <5}: {1: >9}", "SCORE", score);
    let text = graphics::Text::new(
        graphics::TextFragment::new(text)
            .font(asset.font.vt323)
            .scale(PxScale::from(TEXTS_FONT_SIZE))
    );

    graphics::draw(
        ctx,
        &text,
        DrawParam::default()
            .dest([TEXTS_ORIGIN_X, texts_y(0)]),
    )?;

    Ok(())
}

fn draw_removed_line_count(ctx: &mut Context, asset: &Asset, lines: usize) -> GameResult {
    let text = format!("{0: <5}: {1: >9}", "LINES", lines);
    let text = graphics::Text::new(
        graphics::TextFragment::new(text)
            .font(asset.font.vt323)
            .scale(PxScale::from(TEXTS_FONT_SIZE))
    );
    graphics::draw(
        ctx,
        &text,
        DrawParam::default()
            .dest([TEXTS_ORIGIN_X, texts_y(1)]),
    )?;

    Ok(())
}

fn draw_timer(ctx: &mut Context, asset: &Asset, elapsed: &Duration) -> GameResult {
    let min = elapsed.as_secs() / 60;
    let sec = elapsed.as_secs() % 60;
    let milli_sec = elapsed.as_millis() % 100;

    let text = format!("{0: <5}: {1: >03}:{2: >02}:{3: >02}", "TIMER", min, sec, milli_sec);
    let text = graphics::Text::new(
        graphics::TextFragment::new(text)
            .font(asset.font.vt323)
            .scale(PxScale::from(TEXTS_FONT_SIZE))
    );
    graphics::draw(
        ctx,
        &text,
        DrawParam::default()
            .dest([TEXTS_ORIGIN_X, texts_y(2)]),
    )?;

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

// nth is 0-indexed
fn texts_y(nth: usize) -> f32 {
    TEXTS_ORIGIN_Y + (nth as f32) * (TEXTS_FONT_SIZE + TEXTS_PADDING)
}

struct RemovingLineAnimation {
    lines: Vec<usize>,
    elapsed: Duration,
}

const REMOVING_LINE_ANIM_PHASE_1: Duration = Duration::from_secs_f32(0.4);
const REMOVING_LINE_ANIM_PHASE_2: Duration = Duration::from_secs_f32(0.55);

impl RemovingLineAnimation {
    fn new(lines: Vec<usize>) -> RemovingLineAnimation {
        RemovingLineAnimation {
            lines,
            elapsed: Duration::ZERO,
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        let elapsed = self.elapsed;

        if elapsed < REMOVING_LINE_ANIM_PHASE_1 {
            let alpha = 1. - (REMOVING_LINE_ANIM_PHASE_1 - elapsed).as_secs_f32() / REMOVING_LINE_ANIM_PHASE_1.as_secs_f32();
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
        } else if elapsed < REMOVING_LINE_ANIM_PHASE_2 {
            let percentage = (elapsed - REMOVING_LINE_ANIM_PHASE_1).as_secs_f32()
                / (REMOVING_LINE_ANIM_PHASE_2 - REMOVING_LINE_ANIM_PHASE_1).as_secs_f32();
            let color = graphics::Color::new(1., 1., 1., percentage);

            for &line in &self.lines {
                let y = FIELD_ORIGIN_Y + (line as f32 - 1.) * BLOCK_LENGTH
                    + (BLOCK_LENGTH * percentage) / 2.;
                let height = BLOCK_LENGTH - BLOCK_LENGTH * percentage;

                let rect = graphics::Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    Rect::new(
                        FIELD_ORIGIN_X,
                        y,
                        (FIELD_UNIT_WIDTH as f32) * BLOCK_LENGTH,
                        height,
                    ),
                    color,
                )?;

                graphics::draw(ctx, &rect, DrawParam::default())?;
            }
        }

        Ok(())
    }

    fn is_finished(&self) -> bool {
        self.elapsed > REMOVING_LINE_ANIM_PHASE_2
    }
}