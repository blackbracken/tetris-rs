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
    fn new_hold() -> DeviceInput {
        DeviceInput::Hold {
            delta_last_handled: Duration::ZERO,
            delta_from_began: Duration::ZERO,
        }
    }

    pub fn next_state(self, pressed: bool, delta: &Duration) -> DeviceInput {
        use DeviceInput::*;

        if !pressed {
            None
        } else {
            match self {
                None => Push,
                Push => DeviceInput::new_hold(),
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

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(DeviceInput::None, DeviceInput::Push)]
    #[test_case(DeviceInput::Push, DeviceInput::new_hold())]
    #[test_case(DeviceInput::new_hold(), DeviceInput::new_hold())]
    fn test_next_state_if_pressed(src: DeviceInput, ans: DeviceInput) {
        let next_state = src.next_state(true, &Duration::ZERO);
        assert_eq!(next_state, ans);
    }

    #[test_case(DeviceInput::None, DeviceInput::None)]
    #[test_case(DeviceInput::Push, DeviceInput::None)]
    #[test_case(DeviceInput::new_hold(), DeviceInput::None)]
    fn test_next_state_if_not_pressed(src: DeviceInput, ans: DeviceInput) {
        let next_state = src.next_state(false, &Duration::ZERO);
        assert_eq!(next_state, ans);
    }

    #[test]
    fn test_hold_1500ms() {
        let milli_seconds = Duration::new(1, 500_000_000);
        let next_state = DeviceInput::new_hold().next_state(true, &milli_seconds);
        let ans = DeviceInput::Hold {
            delta_from_began: milli_seconds.clone(),
            delta_last_handled: milli_seconds.clone(),
        };

        assert_eq!(next_state, ans);
    }
}
