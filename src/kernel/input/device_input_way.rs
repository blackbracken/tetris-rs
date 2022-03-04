use std::time::Duration;

/// デバイスが検出した入力の経路を表す.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DeviceInputWay {
    /// 空の入力
    None,

    /// 短押し
    Push,

    /// 長押し
    Hold {
        /// 長押しをし始めてからの経過時間
        delta_from_began: Duration,

        /// 最後に長押しを検出してからの経過時間
        delta_last_handled: Duration,
    },
}

impl DeviceInputWay {
    fn new_hold(delta_from_began: Duration, delta_last_handled: Duration) -> DeviceInputWay {
        DeviceInputWay::Hold {
            delta_from_began,
            delta_last_handled,
        }
    }

    fn new_hold_zero() -> DeviceInputWay {
        DeviceInputWay::Hold {
            delta_from_began: Duration::ZERO,
            delta_last_handled: Duration::ZERO,
        }
    }

    pub fn next_state(self, pressed: bool, delta: &Duration) -> DeviceInputWay {
        use DeviceInputWay::*;

        if !pressed {
            None
        } else {
            match self {
                None => Push,
                Push => DeviceInputWay::new_hold(*delta, *delta),
                Hold {
                    delta_from_began,
                    delta_last_handled,
                } => {
                    let delta_from_began = delta_from_began.saturating_add(*delta);
                    let delta_last_handled = delta_last_handled.saturating_add(*delta);

                    DeviceInputWay::Hold {
                        delta_from_began,
                        delta_last_handled,
                    }
                }
            }
        }
    }

    pub fn handled_if_hold(self) -> DeviceInputWay {
        use DeviceInputWay::*;

        match self {
            Hold {
                delta_from_began, ..
            } => DeviceInputWay::new_hold(delta_from_began, Duration::ZERO),
            _ => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(DeviceInputWay::None, DeviceInputWay::Push)]
    #[test_case(DeviceInputWay::Push, DeviceInputWay::new_hold_zero())]
    #[test_case(DeviceInputWay::new_hold_zero(), DeviceInputWay::new_hold_zero())]
    fn test_next_state_if_pressed(src: DeviceInputWay, ans: DeviceInputWay) {
        let next_state = src.next_state(true, &Duration::ZERO);
        assert_eq!(next_state, ans);
    }

    #[test_case(DeviceInputWay::None, DeviceInputWay::None)]
    #[test_case(DeviceInputWay::Push, DeviceInputWay::None)]
    #[test_case(DeviceInputWay::new_hold_zero(), DeviceInputWay::None)]
    fn test_next_state_if_not_pressed(src: DeviceInputWay, ans: DeviceInputWay) {
        let next_state = src.next_state(false, &Duration::ZERO);
        assert_eq!(next_state, ans);
    }

    #[test]
    fn test_hold_1500ms() {
        let milli_seconds = Duration::from_millis(1500);
        let next_state = DeviceInputWay::new_hold_zero().next_state(true, &milli_seconds);
        let ans = DeviceInputWay::Hold {
            delta_from_began: milli_seconds.clone(),
            delta_last_handled: milli_seconds.clone(),
        };

        assert_eq!(next_state, ans);
    }

    #[test]
    fn test_handled_hold() {
        let seconds = Duration::from_secs(1);
        let handled = DeviceInputWay::new_hold_zero()
            .next_state(true, &seconds)
            .handled_if_hold();
        let ans = DeviceInputWay::Hold {
            delta_from_began: seconds.clone(),
            delta_last_handled: Duration::ZERO,
        };

        assert_eq!(handled, ans);
    }
}
