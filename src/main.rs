use ggez::{
    conf::{FullscreenType, NumSamples, WindowMode, WindowSetup},
    ContextBuilder, GameResult,
};

use tetris_rs::{start, WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() -> GameResult {
    start(
        ContextBuilder::new("tetris-rs", "blackbracken")
            .window_mode(WindowMode {
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                maximized: false,
                fullscreen_type: FullscreenType::Windowed,
                borderless: false,
                min_width: 0.,
                min_height: 0.,
                max_width: 0.,
                max_height: 0.,
                resizable: false,
                visible: true,
            })
            .window_setup(WindowSetup {
                title: "tetris-rs".to_owned(),
                samples: NumSamples::Zero,
                vsync: true,
                icon: "".to_owned(),
                srgb: false,
            }),
    )
}
