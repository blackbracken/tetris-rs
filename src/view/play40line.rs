use ggez::{Context, GameResult, graphics};
use ggez::graphics::Color;

use crate::resource::SharedResource;
use crate::router::{Next, ViewState};
use crate::router::ViewState::ForPlay40Line;

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

    graphics::draw(ctx, &resource.block_image, graphics::DrawParam::default());

    graphics::present(ctx)?;

    Ok(())
}