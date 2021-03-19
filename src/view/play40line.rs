use ggez::{Context, GameResult, graphics};
use ggez::graphics::Color;

use crate::resource::SharedResource;
use crate::router::{Next, ViewState};
use crate::router::ViewState::ForPlay40Line;
use rand::Rng;

trait UnitSpace {

}

#[derive(Clone)]
pub struct Play40LineState {}

impl Play40LineState {
    pub fn new(_ctx: &mut Context) -> GameResult<Play40LineState> {
        Ok(Play40LineState {}) // TODO: implement
    }
}

pub fn update(_ctx: &mut Context, state: &Play40LineState) -> Next {
    let new_state = state.clone();
    Next::do_continue(ForPlay40Line { state: new_state })
}

pub fn draw(ctx: &mut Context, state: &Play40LineState, resource: &SharedResource) -> GameResult {
    graphics::clear(ctx, resource.background_color);

    for y in 0..20 {
        for x in 0..10 {
            if rand::random::<u8>() % 10u8 == 0 {
                graphics::draw(ctx, &resource.red_block_image, graphics::DrawParam::default().dest([(x * 32) as f32, (y * 32) as f32]));
            } else {
                graphics::draw(ctx, &resource.block_image, graphics::DrawParam::default().dest([(x * 32) as f32, (y * 32) as f32]));
            }
        }
    }

    graphics::present(ctx)?;

    Ok(())
}