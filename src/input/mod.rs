use bevy::{input::keyboard::Key, prelude::*};

pub fn navigation(key_input: Res<ButtonInput<Key>>) {
    if key_input.just_pressed(Key::ArrowUp) {
        info!("arrow up just pressed");
    }
    if key_input.just_pressed(Key::ArrowDown) {
        info!("arrow down just pressed");
    }

    if key_input.just_pressed(Key::ArrowLeft) {
        info!("arrow left just pressed");
    }
    if key_input.just_pressed(Key::ArrowRight) {
        info!("arrow right just pressed");
    }

    if key_input.just_pressed(Key::Enter) {
        info!("Enter just pressed");
    }
    if key_input.just_pressed(Key::Escape) {
        info!("Escape just pressed");
    }
}
