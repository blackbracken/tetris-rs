use std::{collections::HashMap, time::Duration};

use crate::domain::{control_code::ControlCode, device_input::DeviceInput};

pub struct InputCache {
    input_map: HashMap<ControlCode, DeviceInput>,
}

impl InputCache {
    pub fn new() -> InputCache {
        let input_map = ControlCode::all()
            .into_iter()
            .map(|code| (code, DeviceInput::None))
            .collect();

        InputCache { input_map }
    }

    pub fn handle_inputs(mut self, inputs: &Vec<ControlCode>, delta: &Duration) -> Self {
        use DeviceInput::*;

        let new_input_map = ControlCode::all()
            .into_iter()
            .map(|code| {
                if inputs.contains(&code) {
                    let old_input = (&mut self.input_map)
                        .entry(code.clone())
                        .or_insert(DeviceInput::None);

                    let next_state = match old_input {
                        None => Push,
                        Push => Hold {
                            delta: Duration::ZERO,
                        },
                        Hold { delta: hold_delta } => {
                            DeviceInput::hold(hold_delta.saturating_add(*delta))
                        }
                    };

                    (code, next_state)
                } else {
                    (code, None)
                }
            })
            .collect();

        self.input_map = new_input_map;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cache = InputCache::new();
        let values = cache.input_map.values();

        assert_eq!(values.len(), ControlCode::all().len());

        for input in values {
            assert_eq!(*input, DeviceInput::None);
        }
    }

    #[test]
    fn test_handle_push() {
        let cache = InputCache::new();
        let inputs = vec![ControlCode::MoveLeft];

        let cache = cache.handle_inputs(&inputs, &Duration::ZERO);

        let res = cache.input_map.get(&ControlCode::MoveLeft).unwrap();
        assert_eq!(res, &DeviceInput::Push);

        let otherwise = ControlCode::all()
            .into_iter()
            .filter(|code| code != &ControlCode::MoveLeft)
            .map(|code| cache.input_map.get(&code).unwrap())
            .collect::<Vec<_>>();
        for other in otherwise {
            assert_eq!(other, &DeviceInput::None);
        }
    }

    // TODO: input_mapに直接アクセスしないようにメソッド生やしてテスト増やす
}
