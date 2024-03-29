use ggez::event::{Button, KeyCode};

use crate::model::control_code::ControlCode;

pub trait ControlCodeRepository {
    fn key_codes(&self, code: &ControlCode) -> Vec<KeyCode>;
    fn buttons(&self, code: &ControlCode) -> Vec<Button>;
}
