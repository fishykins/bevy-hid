use bevy::{input::gamepad::GamepadEventRaw, prelude::*};
use hidasp::bindings::*;

pub(crate) struct NativeEvent {
    pub(crate) bind: BindingType,
    pub(crate) state: BindingEvent,
}

pub(crate) enum BindingEvent {
    Pressed,
    Released,
    Value(f32),
}

impl NativeEvent {
    pub(crate) fn to_bevy_event(&self, gamepad: Gamepad) -> GamepadEventRaw {
        match self.bind {
            BindingType::Button(button) => {
                // first, get the bevy button type
                let button_type = get_button(button);
                match self.state {
                    BindingEvent::Released => GamepadEventRaw {
                        gamepad,
                        event_type: GamepadEventType::ButtonChanged(button_type, 0.0),
                    },
                    _ => GamepadEventRaw {
                        gamepad,
                        event_type: GamepadEventType::ButtonChanged(button_type, 1.0),
                    },
                }
<<<<<<< HEAD
            }
=======
            },
>>>>>>> 34fa6a194637a2a4d8449c6395ba241ce4c4bbcb
            BindingType::ButtonGroup(group, button) => {
                let button_type = get_group_button(group, button);
                match self.state {
                    BindingEvent::Released => GamepadEventRaw {
                        gamepad,
                        event_type: GamepadEventType::ButtonChanged(button_type, 0.0),
                    },
                    _ => GamepadEventRaw {
                        gamepad,
                        event_type: GamepadEventType::ButtonChanged(button_type, 1.0),
                    },
                }
<<<<<<< HEAD
            }
=======
            },
>>>>>>> 34fa6a194637a2a4d8449c6395ba241ce4c4bbcb
            BindingType::Axis(axis, index) => {
                let axis_type = get_axis(axis, index as usize);
                let value: f32 = match self.state {
                    BindingEvent::Pressed => 1.0,
                    BindingEvent::Released => 0.0,
                    BindingEvent::Value(v) => v,
                };
                GamepadEventRaw {
                    gamepad,
                    event_type: GamepadEventType::AxisChanged(axis_type, value),
                }
<<<<<<< HEAD
            }
        }
=======
            },
        }
        
>>>>>>> 34fa6a194637a2a4d8449c6395ba241ce4c4bbcb
    }
}

fn get_button(button: ButtonType) -> GamepadButtonType {
    match button {
<<<<<<< HEAD
        ButtonType::Button(i) => GamepadButtonType::Other(i + 2),
        ButtonType::Trigger(i) => GamepadButtonType::Other(i),
        ButtonType::Function(i) => GamepadButtonType::Other(i + 10),
        ButtonType::Up => GamepadButtonType::North,
        ButtonType::Down => GamepadButtonType::South,
        ButtonType::Left => GamepadButtonType::West,
        ButtonType::Right => GamepadButtonType::East,
        ButtonType::UpLeft => todo!(),
        ButtonType::UpRight => todo!(),
        ButtonType::DownLeft => todo!(),
        ButtonType::DownRight => todo!(),
        ButtonType::Press => todo!(),
        ButtonType::Fire => todo!(),
        ButtonType::HalfDepress => todo!(),
    }
}

fn get_group_button(_group: ButtonGroupType, button: ButtonType) -> GamepadButtonType {
    match button {
        ButtonType::Button(i) => GamepadButtonType::Other(i + 2),
        ButtonType::Trigger(i) => GamepadButtonType::Other(i),
        ButtonType::Function(i) => GamepadButtonType::Other(i + 10),
        ButtonType::Up => GamepadButtonType::North,
        ButtonType::Down => GamepadButtonType::South,
        ButtonType::Left => GamepadButtonType::West,
        ButtonType::Right => GamepadButtonType::East,
        ButtonType::UpLeft => GamepadButtonType::Z,
        ButtonType::UpRight => GamepadButtonType::Z,
        ButtonType::DownLeft => GamepadButtonType::Z,
        ButtonType::DownRight => GamepadButtonType::Z,
        ButtonType::Press => GamepadButtonType::Z,
        ButtonType::Fire => GamepadButtonType::Other(2),
        ButtonType::HalfDepress => GamepadButtonType::Other(1),
=======
    ButtonType::Button(i) => GamepadButtonType::Other(i + 2),
    ButtonType::Trigger(i) => GamepadButtonType::Other(i),
    ButtonType::Function(i) => GamepadButtonType::Other(i + 10),
    ButtonType::Up => GamepadButtonType::North,
    ButtonType::Down => GamepadButtonType::South,
    ButtonType::Left => GamepadButtonType::West,
    ButtonType::Right => GamepadButtonType::East,
    ButtonType::UpLeft => todo!(),
    ButtonType::UpRight => todo!(),
    ButtonType::DownLeft => todo!(),
    ButtonType::DownRight => todo!(),
    ButtonType::Press => todo!(),
    ButtonType::Fire => todo!(),
    ButtonType::HalfDepress => todo!(),
    }
}


fn get_group_button(_group: ButtonGroupType, button: ButtonType) -> GamepadButtonType {
    match button {
    ButtonType::Button(i) => GamepadButtonType::Other(i + 2),
    ButtonType::Trigger(i) => GamepadButtonType::Other(i),
    ButtonType::Function(i) => GamepadButtonType::Other(i + 10),
    ButtonType::Up => GamepadButtonType::North,
    ButtonType::Down => GamepadButtonType::South,
    ButtonType::Left => GamepadButtonType::West,
    ButtonType::Right => GamepadButtonType::East,
    ButtonType::UpLeft => GamepadButtonType::Z,
    ButtonType::UpRight => GamepadButtonType::Z,
    ButtonType::DownLeft => GamepadButtonType::Z,
    ButtonType::DownRight => GamepadButtonType::Z,
    ButtonType::Press => GamepadButtonType::Z,
    ButtonType::Fire => GamepadButtonType::Other(2),
    ButtonType::HalfDepress => GamepadButtonType::Other(1),
>>>>>>> 34fa6a194637a2a4d8449c6395ba241ce4c4bbcb
    }
}

fn get_axis(axis: AxisType, index: usize) -> GamepadAxisType {
    match axis {
        AxisType::X => GamepadAxisType::LeftStickX,
        AxisType::Y => GamepadAxisType::LeftStickY,
        AxisType::Z => GamepadAxisType::RightStickX,
        AxisType::RZ => GamepadAxisType::RightStickY,
        AxisType::LeftStick => {
            if index == 0 {
                GamepadAxisType::LeftStickX
            } else {
                GamepadAxisType::LeftStickY
            }
<<<<<<< HEAD
        }
=======
        },
>>>>>>> 34fa6a194637a2a4d8449c6395ba241ce4c4bbcb
        AxisType::RightStick => {
            if index == 0 {
                GamepadAxisType::RightStickX
            } else {
                GamepadAxisType::RightStickY
            }
<<<<<<< HEAD
        }
        AxisType::LeftTrigger => GamepadAxisType::LeftZ,
        AxisType::RightTrigger => GamepadAxisType::RightZ,
        AxisType::FlightStick => match index {
            0 => GamepadAxisType::LeftStickX,
            _ => GamepadAxisType::LeftStickY,
=======
        },
        AxisType::LeftTrigger => GamepadAxisType::LeftZ,
        AxisType::RightTrigger => GamepadAxisType::RightZ,
        AxisType::FlightStick => {
            match index {
                0 => GamepadAxisType::LeftStickX,
                _ => GamepadAxisType::LeftStickY,
            }
>>>>>>> 34fa6a194637a2a4d8449c6395ba241ce4c4bbcb
        },
        AxisType::FLightStickTwist => match index {
            0 => GamepadAxisType::LeftStickX,
            1 => GamepadAxisType::LeftStickY,
            _ => GamepadAxisType::RightStickX,
<<<<<<< HEAD
        },
        AxisType::Wheel => GamepadAxisType::LeftStickX,
        AxisType::Throttle(v) => GamepadAxisType::Other(v),
        AxisType::Stick(v) => GamepadAxisType::Other(v),
        AxisType::Slider(v) => GamepadAxisType::Other(v + 10),
        AxisType::Rotor(v) => GamepadAxisType::Other(v + 20),
        AxisType::Trigger(v) => GamepadAxisType::Other(v + 30),
        AxisType::Other(v) => GamepadAxisType::Other(v + 40),
    }
}
=======
        }
        AxisType::Wheel => GamepadAxisType::LeftStickX,
        AxisType::Throttle(v) => GamepadAxisType::Other(v),
        AxisType::Stick(v) => GamepadAxisType::Other(v),
        AxisType::Trigger(v) => GamepadAxisType::Other(v),
        AxisType::Slider(v) => GamepadAxisType::Other(v),
        AxisType::Rotor(v) => GamepadAxisType::Other(v),
        AxisType::Other(v) => GamepadAxisType::Other(v),
    }
}
>>>>>>> 34fa6a194637a2a4d8449c6395ba241ce4c4bbcb
