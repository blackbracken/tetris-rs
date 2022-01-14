use enum_iterator::IntoEnumIterator;

#[derive(IntoEnumIterator, PartialEq)]
pub enum ControlCode {
    // In-game
    MoveLeft,
    MoveRight,
    SoftDrop,
    HardDrop,
    RotateCounterclockwise,
    RotateClockwise,
    SwapHold,

    // Menu
    MenuUp,
    MenuDown,
    MenuRight,
    MenuLeft,
    MenuEnter,
    MenuBack,
}

impl ControlCode {
    pub fn all() -> Vec<ControlCode> {
        ControlCode::into_enum_iter().collect()
    }
}
