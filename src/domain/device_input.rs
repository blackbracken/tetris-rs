use std::time::Duration;

use crate::domain::control_code::ControlCode;

#[derive(Debug, Eq, PartialEq)]
pub enum DeviceInput {
    None,
    Push,
    Hold { delta: Duration },
}

impl DeviceInput {
    pub fn hold(delta: Duration) -> DeviceInput {
        DeviceInput::Hold { delta }
    }

    fn inputs(&self) -> bool {
        unimplemented!()
    }
}
