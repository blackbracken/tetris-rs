use ggez::{Context, GameResult, graphics};

use crate::asset::Asset;
use crate::router::Next;
use crate::router::SceneState::ForPlay40Line;
use crate::tetris::game::{Game, MinoBlock};

pub struct Play40LineState {
    game: Game
}

impl Play40LineState {
    pub fn new(_ctx: &mut Context) -> GameResult<Play40LineState> {
        Ok(
            Play40LineState {
                game: Game::new(),
            }
        )
    }
}

pub fn init(_ctx: &mut Context, asset: &mut Asset) {
    asset.audio.stop_bgm();
}

pub fn update(_ctx: &mut Context, mut state: Play40LineState) -> Next {
    Next::do_continue(ForPlay40Line { state })
}

pub fn draw(ctx: &mut Context, state: &Play40LineState, asset: &mut Asset) -> GameResult {
    graphics::clear(ctx, asset.color.background);

    let field = state.game.board.field();
    for y in 0..20 {
        for x in 0..10 {
            let block = field
                .get(y)
                .and_then(|array| array.get(x))
                .unwrap();

            let img = asset.image.mino_block(ctx, block)?;
            graphics::draw(
                ctx,
                img,
                graphics::DrawParam::default()
                    .dest([(x * 32) as f32, (y * 32) as f32]),
            );
        }
    }

    graphics::present(ctx)?;

    Ok(())
}