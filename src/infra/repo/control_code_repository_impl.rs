use derive_new::new;
use ggez::event::{Button, KeyCode};

use crate::kernel::{
    input::control_code::ControlCode,
    repo::control_code_repository::ControlCodeRepository,
};

#[derive(new)]
pub struct ControlCodeRepositoryImpl;

impl ControlCodeRepository for ControlCodeRepositoryImpl {
    fn key_codes(&self, code: &ControlCode) -> Vec<KeyCode> {
        use ControlCode::*;
        use KeyCode::*;

        let up = vec![W, Up];
        let down = vec![S, Down];
        let right = vec![D, Right];
        let left = vec![A, Left];

        match code {
            MoveLeft => left,
            MoveRight => right,
            SoftDrop => down,
            HardDrop => up,
            RotateCounterclockwise => vec![J],
            RotateClockwise => vec![K],
            SwapHold => vec![Space],
            MenuUp => up,
            MenuDown => down,
            MenuRight => right,
            MenuLeft => left,
            MenuEnter => vec![Space, Return],
            MenuBack => vec![Escape],
        }
    }

    fn buttons(&self, code: &ControlCode) -> Vec<Button> {
        use Button::*;
        use ControlCode::*;

        match code {
            MoveLeft => vec![DPadLeft],
            MoveRight => vec![DPadRight],
            SoftDrop => vec![DPadDown],
            HardDrop => vec![DPadUp],
            RotateCounterclockwise => vec![South],
            RotateClockwise => vec![East],
            SwapHold => vec![LeftTrigger],
            MenuUp => vec![DPadUp],
            MenuDown => vec![DPadDown],
            MenuRight => vec![DPadRight],
            MenuLeft => vec![DPadLeft],
            MenuEnter => vec![East],
            MenuBack => vec![Start, Select],
        }
    }
}
