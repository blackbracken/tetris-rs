use std::{collections::HashMap, mem::take, time::Duration};

use ggez::{
    graphics,
    graphics::{Color, DrawParam, PxScale, Text, TextFragment},
    Context,
    GameResult,
};
use indoc::indoc;
use rand::random;

use crate::{
    asset::audio::Bgm,
    model::xytuple::F32XYTuple,
    scene::{
        animation_property::{AnimationProperties, AnimationProperty},
        timer::Timer,
        title::selected_item::SelectedItem,
    },
    Asset,
    ControlCode,
    InputCache,
    Next,
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

static TITLE_ASCII: &str = indoc!(
    r"
 __           __
/\ \__       /\ \__         __
\ \ ,_\    __\ \ ,_\  _ __ /\_\    ____           _ __   ____
 \ \ \/  /'__`\ \ \/ /\`'__\/\ \  /',__\  _______/\`'__\/',__\
  \ \ \_/\  __/\ \ \_\ \ \/ \ \ \/\__, `\/\______\ \ \//\__, `\
   \ \__\ \____\\ \__\\ \_\  \ \_\/\____/\/______/\ \_\\/\____/
    \/__/\/____/ \/__/ \/_/   \/_/\/___/           \/_/ \/___/
    "
);
const PARTICLE_SIZE: f32 = 10.;

#[derive(new)]
pub struct TitleState {
    draw_state: TitleDrawState,
    cursor: SelectedItem,
}

pub struct TitleDrawState {
    title_texts_ascii: Vec<Text>,
    selected_item_text_hash: HashMap<SelectedItem, Text>,
    animation_properties: AnimationProperties<StarParticle>,
    particle_add_timer: Timer,
}

#[derive(Default)]
struct StarParticle {
    duration: Duration,
    limit: Duration,
    start_pos: F32XYTuple,
    start_rot: f32,
    y_spd: f32,
    rot_spd: f32,
}

impl StarParticle {
    fn new(start_pos: F32XYTuple, start_rot: f32, y_spd: f32, rot_spd: f32) -> Self {
        StarParticle {
            duration: Duration::ZERO,
            limit: Duration::from_secs(5),
            start_pos,
            start_rot,
            y_spd,
            rot_spd,
        }
    }

    fn gen_randomized() -> Self {
        let pos = (
            WINDOW_WIDTH * random::<f32>(),
            WINDOW_HEIGHT * random::<f32>(),
        )
            .into();

        let start_rot = 360f32.to_radians() * random::<f32>();
        let y_spd = 36. * random::<f32>() + 4.;
        let rot_spd = 90f32.to_radians() * random::<f32>() + 30f32.to_radians();

        StarParticle::new(pos, start_rot, y_spd, rot_spd)
    }

    fn pos(&self) -> F32XYTuple {
        (
            self.start_pos.x,
            self.start_pos.y - self.y_spd * self.duration.as_secs_f32(),
        )
            .into()
    }

    fn rot(&self) -> f32 {
        (self.start_rot + (self.rot_spd * (self.duration.as_secs_f32() % 360f32.to_radians())))
            % 360f32.to_radians()
    }
}

impl AnimationProperty for StarParticle {
    fn duration(&self) -> &Duration {
        &self.duration
    }

    fn elapse(mut self, delta: &Duration) -> Self {
        let duration = self.duration.saturating_add(delta.clone());
        self.duration = duration;

        self
    }

    fn is_active(&self) -> bool {
        self.pos().y > -PARTICLE_SIZE
    }
}

impl TitleDrawState {
    fn new() -> TitleDrawState {
        let title_texts_ascii = TITLE_ASCII
            .split("\n")
            .into_iter()
            .map(|line| Text::new(TextFragment::from(line)))
            .collect();

        let selected_item_text_hash = SelectedItem::all()
            .into_iter()
            .map(|item| {
                let name = item.name().to_owned();

                (
                    item,
                    Text::new(TextFragment::new(name).scale(PxScale::from(32.))),
                )
            })
            .collect();

        let mut ap = AnimationProperties::new();
        for _ in 1..=10 {
            ap.add(StarParticle::gen_randomized());
        }

        TitleDrawState {
            title_texts_ascii,
            selected_item_text_hash,
            animation_properties: ap,
            particle_add_timer: Timer::infinite(Duration::from_secs(1), Duration::from_secs(2)),
        }
    }
}

pub fn init(ctx: &mut Context, asset: &mut Asset) -> GameResult<TitleState> {
    asset.audio.play_bgm(ctx, Bgm::Title)?;

    Ok(TitleState {
        draw_state: TitleDrawState::new(),
        cursor: SelectedItem::PlayFortyLine,
    })
}

pub fn update(
    _: &mut Context,
    input_cache: &mut InputCache,
    state: TitleState,
    delta: &Duration,
) -> GameResult<Next> {
    let mut state = state;

    let mut props = take(&mut state.draw_state.animation_properties);
    props.elapse(delta);
    if state.draw_state.particle_add_timer.consume_if_beep() {
        let mut r = StarParticle::gen_randomized();
        r.start_pos.y = WINDOW_HEIGHT + PARTICLE_SIZE;
        props.add(r);
    }
    state.draw_state.animation_properties = props;
    state.draw_state.particle_add_timer.elapse(*delta);

    if input_cache.has_pushed(&ControlCode::MenuUp) {
        if let Some(prev) = state.cursor.prev() {
            state.cursor = prev;
        }
    }
    if input_cache.has_pushed(&ControlCode::MenuDown) {
        if let Some(next) = state.cursor.next() {
            state.cursor = next;
        }
    }
    if input_cache.has_pushed(&ControlCode::MenuEnter) {
        match state.cursor {
            SelectedItem::PlayFortyLine => unimplemented!(),
            SelectedItem::Exit => return Ok(Next::exit()),
        }
    }

    Ok(Next::do_continue(state.into()))
}

pub fn draw(ctx: &mut Context, state: &TitleState, asset: &mut Asset) -> GameResult {
    let draw_state = &state.draw_state;

    graphics::clear(ctx, asset.color.background);

    for star in state.draw_state.animation_properties.props() {
        let (x, y): (f32, f32) = star.pos().into();
        let rot = star.rot();

        graphics::draw(
            ctx,
            &asset.image.title_particle,
            graphics::DrawParam::default()
                .color(Color::new(1., 1., 1., 0.2))
                .dest([x, y])
                .scale([0.5, 0.5])
                .rotation(rot)
                .offset([0.5, 0.5]),
        )?;
    }

    let title_max_width = draw_state.title_texts_ascii.get(4).unwrap().width(ctx);
    for (idx, text) in draw_state.title_texts_ascii.iter().enumerate() {
        let x = WINDOW_WIDTH / 2. - title_max_width / 2.;
        let y = 50. + (15 * idx) as f32;

        graphics::draw(ctx, text, DrawParam::default().dest([x, y]))?;
    }

    for (idx, item) in SelectedItem::all().iter().enumerate() {
        let text = draw_state.selected_item_text_hash.get(item).unwrap();
        let x = WINDOW_WIDTH / 2. - text.width(ctx) / 2.;
        let y = WINDOW_HEIGHT / 3. + (50 * idx) as f32;

        graphics::draw(ctx, text, graphics::DrawParam::default().dest([x, y]))?;

        if item == &state.cursor {
            let cursor_scale = 0.5;
            let cursor_x = x - 30.;
            let cursor_y = y + text.height(ctx) / 2.
                - f32::from(asset.image.cursor.height()) * cursor_scale / 2.;

            graphics::draw(
                ctx,
                &asset.image.cursor,
                DrawParam::default()
                    .dest([cursor_x, cursor_y])
                    .scale([cursor_scale, cursor_scale]),
            )?;
        }
    }

    graphics::present(ctx)?;

    Ok(())
}
