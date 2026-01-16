use bevy::{
    input::gamepad::{GamepadAxis, GamepadButton}, reflect::Reflect
};
use serde::Deserialize;

/// The main identifier of an input type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Reflect)]
pub enum Binding {
    Button(GamepadButton),
    Axis(GamepadAxis),
}

impl From<GamepadButton> for Binding {
    fn from(button: GamepadButton) -> Self {
        Self::Button(button)
    }
}

impl From<GamepadAxis> for Binding {
    fn from(axis: GamepadAxis) -> Self {
        Self::Axis(axis)
    }
}
