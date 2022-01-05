use crate::domain::repo::control_code_repository::ControlCodeRepository;
use ggez::{
    event::{Button, KeyCode},
    input::{gamepad::gamepads, keyboard},
    Context,
};

pub enum ControlCode {
    // In-game
    MoveLeft,
    MoveRight,
    SoftDrop,
    HardDrop,
    RotateCounterclockwise,
    RotateClockwise,
    SwapHold,

    // Menu
    MenuUp,
    MenuDown,
    MenuRight,
    MenuLeft,
    MenuEnter,
    MenuBack,
}
