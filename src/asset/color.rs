use ggez::graphics;

pub struct Color {
    pub background: graphics::Color,
    pub panel: graphics::Color,
    pub separator: graphics::Color,
    pub grid_line: graphics::Color,
    pub frame: graphics::Color,
}

impl Color {
    pub(super) fn new() -> Color {
        Color {
            background: graphics::Color::from_rgb(24, 24, 24),
            panel: graphics::Color::from_rgba(128, 255, 255, 32),
            separator: graphics::Color::from_rgb(64, 64, 64),
            grid_line: graphics::Color::from_rgba(24, 24, 24, 128),
            frame: graphics::Color::from_rgb(148, 148, 148),
        }
    }
}
