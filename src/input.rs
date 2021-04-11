use ggez::event::KeyCode;
use ggez::input::gamepad::gamepads;
use ggez::input::gamepad::gilrs::Button;
use ggez::input::keyboard;
use ggez::Context;

pub fn pressed_enter(ctx: &Context) -> bool {
    pressed_either(ctx, &[KeyCode::Return, KeyCode::Space], &[Button::East])
}

pub fn pressed_pause(ctx: &Context) -> bool {
    pressed_either(ctx, &[KeyCode::Escape], &[Button::Start, Button::Select])
}

pub fn pressed_up(ctx: &Context) -> bool {
    pressed_either(ctx, &[KeyCode::Up, KeyCode::W], &[Button::DPadUp])
}

pub fn pressed_down(ctx: &Context) -> bool {
    pressed_either(ctx, &[KeyCode::Down, KeyCode::S], &[Button::DPadDown])
}

pub fn pressed_move_left(ctx: &Context) -> bool {
    pressed_either(ctx, &[KeyCode::A], &[Button::DPadLeft])
}

pub fn pressed_move_right(ctx: &Context) -> bool {
    pressed_either(ctx, &[KeyCode::D], &[Button::DPadRight])
}

pub fn pressed_spin_left(ctx: &Context) -> bool {
    pressed_either(ctx, &[KeyCode::J], &[Button::South])
}

pub fn pressed_spin_right(ctx: &Context) -> bool {
    pressed_either(ctx, &[KeyCode::K], &[Button::East])
}

pub fn pressed_hold(ctx: &Context) -> bool {
    pressed_either(ctx, &[KeyCode::Space], &[Button::LeftTrigger])
}

fn pressed_either(ctx: &Context, keys: &[KeyCode], buttons: &[Button]) -> bool {
    let on_keyboard = keys.iter().any(|&key| keyboard::is_key_pressed(ctx, key));
    let on_pad = buttons
        .iter()
        .any(|&btn| gamepads(ctx).any(|(_, pad)| pad.is_pressed(btn)));

    on_keyboard || on_pad
}
