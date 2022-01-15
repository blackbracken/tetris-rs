use std::time::Duration;

use crate::domain::control_code::ControlCode;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DeviceInput {
    None,
    Push,
    Hold {
        delta_from_began: Duration,
        delta_last_handled: Duration,
    },
}

impl DeviceInput {
    pub fn next_state(self, pressed: bool, delta: &Duration) -> DeviceInput {
        use DeviceInput::*;

        if !pressed {
            None
        } else {
            match self {
                None => Push,
                Push => DeviceInput::Hold {
                    delta_last_handled: Duration::ZERO,
                    delta_from_began: Duration::ZERO,
                },
                Hold {
                    delta_from_began,
                    delta_last_handled,
                } => {
                    let delta_from_began = delta_from_began.saturating_add(*delta);
                    let delta_last_handled = delta_last_handled.saturating_add(*delta);

                    DeviceInput::Hold {
                        delta_from_began,
                        delta_last_handled,
                    }
                }
            }
        }
    }
}
