use std::{collections::HashMap, time::Duration};

use crate::model::{control_code::ControlCode, device_input::DeviceInput};

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

    pub fn receive_inputs(&mut self, inputs: &Vec<ControlCode>, delta: &Duration) {
        let new_input_map = ControlCode::all()
            .into_iter()
            .map(|code| {
                let old_input = self
                    .input_map
                    .entry(code.clone())
                    .or_insert(DeviceInput::None);

                (code, old_input.next_state(inputs.contains(&code), delta))
            })
            .collect();

        self.input_map = new_input_map;
    }

    pub fn has_none(&self, code: &ControlCode) -> bool {
        match self.input_map.get(code) {
            Some(DeviceInput::None) => true,
            _ => false,
        }
    }

    pub fn has_pushed(&self, code: &ControlCode) -> bool {
        match self.input_map.get(code) {
            Some(DeviceInput::Push) => true,
            _ => false,
        }
    }

    pub fn handle_hold_if_unhandled_yet_after(
        &mut self,
        code: &ControlCode,
        duration: &Duration,
    ) -> bool {
        let should_handle = self.has_hold_unhandled_yet_after(code, duration);
        if should_handle {
            self.handle_if_hold(code);
        }

        should_handle
    }

    pub fn handle_hold_if_handled_before(
        &mut self,
        code: &ControlCode,
        duration: &Duration,
    ) -> bool {
        let should_handle = self.has_hold_handled_before(code, duration);
        if should_handle {
            self.handle_if_hold(code);
        }

        should_handle
    }

    fn has_hold_unhandled_yet_after(&self, code: &ControlCode, duration: &Duration) -> bool {
        match self.input_map.get(code) {
            Some(DeviceInput::Hold {
                delta_from_began,
                delta_last_handled,
            }) if delta_from_began == delta_last_handled && duration <= delta_from_began => true,
            _ => false,
        }
    }

    fn has_hold_handled_before(&self, code: &ControlCode, duration: &Duration) -> bool {
        match self.input_map.get(code) {
            Some(DeviceInput::Hold {
                delta_from_began,
                delta_last_handled,
            }) if delta_last_handled < delta_from_began && duration <= delta_last_handled => true,
            _ => false,
        }
    }

    fn handle_if_hold(&mut self, code: &ControlCode) {
        let input_map = &mut self.input_map;

        if let Some(input @ DeviceInput::Hold { .. }) = input_map.get(code) {
            input_map.insert(code.clone(), input.handled_if_hold());
        }
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
    fn test_none() {
        let mut cache = InputCache::new();
        let inputs: Vec<ControlCode> = Vec::new();

        cache.receive_inputs(&inputs, &Duration::ZERO);

        for code in ControlCode::all() {
            assert!(cache.has_none(&code));
        }
    }

    #[test]
    fn test_push() {
        let mut cache = InputCache::new();
        let inputs = vec![ControlCode::MoveLeft, ControlCode::MenuLeft];

        cache.receive_inputs(&inputs, &Duration::ZERO);

        for pushed_code in &inputs {
            assert!(cache.has_pushed(pushed_code));
        }

        let otherwise = ControlCode::all()
            .into_iter()
            .filter(|code| !inputs.contains(code))
            .collect::<Vec<_>>();

        for not_pushed_code in &otherwise {
            assert!(cache.has_none(not_pushed_code));
        }
    }

    #[test]
    fn test_hold_unhandled_yet_after_1500ms() {
        let mut cache = InputCache::new();
        let inputs = vec![ControlCode::MoveLeft];
        let milli_seconds = Duration::from_millis(1500);

        cache.receive_inputs(&inputs, &milli_seconds);
        cache.receive_inputs(&inputs, &milli_seconds);

        assert!(cache.has_hold_unhandled_yet_after(&ControlCode::MoveLeft, &milli_seconds));
    }

    // ┌──────┐    ┌─────────────┐    ┌────────────────┐    ┌────────────────┐    ┌────────────────┐    ┌────────────────┐
    // │ push ├───►│ receive(1s) ├───►│ receive(500ms) ├───►│ receive(500ms) ├───►│ receive(500ms) ├───►│ receive(500ms) │
    // └──────┘    └──────┬──────┘    └────────────────┘    └───────┬────────┘    └────────────────┘    └───────┬────────┘
    //                    │                                         │                                           │
    //                    │◄───────────────── 1s ──────────────────►│◄─────────────────── 1s ──────────────────►│
    //                    │                                         │                                           │
    //                 handled                                   handled                                     handled
    #[test]
    fn test_hold_handled_before_1000ms() {
        let mut cache = InputCache::new();
        let input = ControlCode::MoveLeft;
        let inputs = vec![ControlCode::MoveLeft];

        let duration_1s = Duration::from_secs(1);
        let duration_500ms = Duration::from_millis(500);

        cache.receive_inputs(&inputs, &duration_1s);

        cache.receive_inputs(&inputs, &duration_1s);
        cache.handle_if_hold(&input);

        cache.receive_inputs(&inputs, &duration_500ms);
        assert!(!cache.handle_hold_if_handled_before(&input, &duration_1s));

        cache.receive_inputs(&inputs, &duration_500ms);
        assert!(cache.handle_hold_if_handled_before(&input, &duration_1s));

        cache.receive_inputs(&inputs, &duration_500ms);
        assert!(!cache.handle_hold_if_handled_before(&input, &duration_1s));

        cache.receive_inputs(&inputs, &duration_500ms);
        assert!(cache.handle_hold_if_handled_before(&input, &duration_1s));
    }
}
