use bevy::prelude::*;
use bevy_hid::HidPlugin;

fn main() {
    App::new().add_plugins((DefaultPlugins, HidPlugin)).run();
}
