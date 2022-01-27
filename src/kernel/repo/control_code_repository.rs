use ggez::event::{Button, KeyCode};

use crate::kernel::input::control_code::ControlCode;

pub trait ControlCodeRepository {
    fn key_codes(&self, code: &ControlCode) -> Vec<KeyCode>;
    fn buttons(&self, code: &ControlCode) -> Vec<Button>;
}
