use crate::domain::control_code::ControlCode;
use ggez::{
    event::{Button, KeyCode},
    input::{gamepad::gamepads, keyboard},
    Context,
};

pub trait ControlCodeRepository {
    fn key_codes(&self, code: &ControlCode) -> Vec<KeyCode>;
    fn buttons(&self, code: &ControlCode) -> Vec<Button>;
}
