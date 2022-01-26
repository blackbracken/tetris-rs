use std::time::Duration;

use ggez::{Context, GameResult, graphics::Image};

use crate::model::xytuple::F32XYTuple;

pub struct AnimatedImage<'a, F, G>
    where
        F: Fn(&Self, &Duration) -> bool,
        G: Fn(&Self, &Duration, &mut Context) -> GameResult,
{
    pub origin: F32XYTuple,
    pub image: &'a Image,
    is_expired: F,
    draw: G,
}

impl<F, G> AnimatedImage<'_, F, G>
    where
        F: Fn(&Self, &Duration) -> bool,
        G: Fn(&Self, &Duration, &mut Context) -> GameResult,
{
    pub fn is_expired(&self, delta: &Duration) -> bool {
        (self.is_expired)(self, delta)
    }

    pub fn draw(&self, delta: &Duration, ctx: &mut Context) -> GameResult {
        (self.draw)(self, delta, ctx)
    }
}
