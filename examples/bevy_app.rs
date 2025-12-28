use bevy::prelude::*;
use bevy_hid::{HidPlugin, buffers::HidBuffer};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HidPlugin))
        .add_systems(Update, check_devices)
        .run();
}

fn check_devices(gamepads: Query<&Gamepad, With<HidBuffer>>) {
    for gamepad in gamepads.iter() {
        for axis in gamepad.get_analog_axes() {
            match axis {
                bevy::input::gamepad::GamepadInput::Axis(gamepad_axis) => match gamepad_axis {
                    GamepadAxis::Other(_) => {
                        println!("Axis: {:?}", gamepad.get_unclamped(*axis))
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
