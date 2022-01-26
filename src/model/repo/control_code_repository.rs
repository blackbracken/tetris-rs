use crate::model::control_code::ControlCode;
use ggez::{
    event::{Button, KeyCode},
};

pub trait ControlCodeRepository {
    fn key_codes(&self, code: &ControlCode) -> Vec<KeyCode>;
    fn buttons(&self, code: &ControlCode) -> Vec<Button>;
}
