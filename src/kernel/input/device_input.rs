use std::time::Duration;

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
    fn new_hold(delta_from_began: Duration, delta_last_handled: Duration) -> DeviceInput {
        DeviceInput::Hold {
            delta_from_began,
            delta_last_handled,
        }
    }

    fn new_hold_zero() -> DeviceInput {
        DeviceInput::Hold {
            delta_from_began: Duration::ZERO,
            delta_last_handled: Duration::ZERO,
        }
    }

    pub fn next_state(self, pressed: bool, delta: &Duration) -> DeviceInput {
        use DeviceInput::*;

        if !pressed {
            None
        } else {
            match self {
                None => Push,
                Push => DeviceInput::new_hold(*delta, *delta),
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

    pub fn handled_if_hold(self) -> DeviceInput {
        use DeviceInput::*;

        match self {
            Hold {
                delta_from_began, ..
            } => DeviceInput::new_hold(delta_from_began, Duration::ZERO),
            _ => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(DeviceInput::None, DeviceInput::Push)]
    #[test_case(DeviceInput::Push, DeviceInput::new_hold_zero())]
    #[test_case(DeviceInput::new_hold_zero(), DeviceInput::new_hold_zero())]
    fn test_next_state_if_pressed(src: DeviceInput, ans: DeviceInput) {
        let next_state = src.next_state(true, &Duration::ZERO);
        assert_eq!(next_state, ans);
    }

    #[test_case(DeviceInput::None, DeviceInput::None)]
    #[test_case(DeviceInput::Push, DeviceInput::None)]
    #[test_case(DeviceInput::new_hold_zero(), DeviceInput::None)]
    fn test_next_state_if_not_pressed(src: DeviceInput, ans: DeviceInput) {
        let next_state = src.next_state(false, &Duration::ZERO);
        assert_eq!(next_state, ans);
    }

    #[test]
    fn test_hold_1500ms() {
        let milli_seconds = Duration::from_millis(1500);
        let next_state = DeviceInput::new_hold_zero().next_state(true, &milli_seconds);
        let ans = DeviceInput::Hold {
            delta_from_began: milli_seconds.clone(),
            delta_last_handled: milli_seconds.clone(),
        };

        assert_eq!(next_state, ans);
    }

    #[test]
    fn test_handled_hold() {
        let seconds = Duration::from_secs(1);
        let handled = DeviceInput::new_hold_zero()
            .next_state(true, &seconds)
            .handled_if_hold();
        let ans = DeviceInput::Hold {
            delta_from_began: seconds.clone(),
            delta_last_handled: Duration::ZERO,
        };

        assert_eq!(handled, ans);
    }
}
