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