use crate::{
    event_mapping::{BindingEvent::*, NativeEvent},
    systems::valid_buffer,
    DeviceBuffer,
};
use hidasp::{
    bindings::*,
    maps::{AxisMap, BufferMap, ButtonMap, Buttonquery},
    HumanInterfaceDevice,
};
use std::collections::HashSet;

pub(crate) fn transcode_device(
    cfg: &HumanInterfaceDevice,
    map: &BufferMap,
    buf_new: Option<&DeviceBuffer>,
    buf_last: Option<&DeviceBuffer>,
) -> Vec<NativeEvent> {
    if !valid_buffer(buf_new) {
        return Vec::new();
    }

    let mut events: Vec<NativeEvent> = Vec::new();

    if let Some(buf_new) = buf_new {
        if let Some(buf_last) = buf_last {
            let mut changed_axis = HashSet::new();

            for (i, binds) in map.iter() {
                let i = *i as usize;
                let last = buf_last[i];
                let new = buf_new[i];

                if last != new {
                    for bind in binds.iter() {
                        // Note: Unwraps from here on out should be safe as the map we are using is provided by a trusted source.
                        match bind {
                            BindingType::Button(button_type) => {
                                // Get the data for this button.
                                let button_map = cfg.map.buttons.get(button_type).unwrap();
                                let pressed_new = check_button(new, button_map);
                                let pressed_last = check_button(last, button_map);
                                if pressed_new != pressed_last {
                                    if pressed_new {
                                        events.push(NativeEvent {
                                            bind: *bind,
                                            state: Pressed,
                                        })
                                    } else {
                                        events.push(NativeEvent {
                                            bind: *bind,
                                            state: Released,
                                        })
                                    }
                                }
                            }
                            BindingType::ButtonGroup(group_type, button_type) => {
                                let button_group = cfg.map.button_groups.get(&group_type).unwrap();
                                let button_map = button_group.buttons.get(&button_type).unwrap(); // God this feels naughty
                                let pressed_new = check_button(new, button_map);
                                let pressed_last = check_button(last, button_map);
                                if pressed_new != pressed_last {
                                    if pressed_new {
                                        events.push(NativeEvent {
                                            bind: *bind,
                                            state: Pressed,
                                        })
                                    } else {
                                        events.push(NativeEvent {
                                            bind: *bind,
                                            state: Released,
                                        })
                                    }
                                }
                            }
                            BindingType::Axis(axis_type, _) => {
                                // Seeing as axis can cover multiple parts of the buffer, simply notify superiors about
                                // the change and move on. Maybe refine this later if we need to optimize.
                                changed_axis.insert(axis_type);
                            }
                        }
                    }
                }
            }

            // Loop over any changed axis
            for axis_type in changed_axis {
                let axis_group = cfg.map.analog_groups.get(axis_type).unwrap();
                // This is wild stuff, but again, we can trust the code is safe.
                let axis = axis_group.as_vec();

                for (i, a) in axis.iter().enumerate() {
                    let max = a.octaves as u16 * 255;
                    let last = calc_axis(buf_last, a);
                    let new = calc_axis(buf_new, a);
                    if last != new {
                        events.push(NativeEvent {
                            bind: BindingType::Axis(*axis_type, i as u8),
                            state: Value((new as f32 / max as f32) * 2.0 - 1.0),
                        })
                    }
                }
            }
        }
    }
    return events;
}

fn check_button(bufv: u8, button: &ButtonMap) -> bool {
    match button.query {
        Buttonquery::Bit(v) => {
            // bitwise comparison
            bufv & v > 0
        }
        Buttonquery::Eq(v) => {
            // equality comparison
            bufv == v
        }
    }
}

fn calc_axis(buffer: &DeviceBuffer, map: &AxisMap) -> u16 {
    let fine = buffer[map.fine as usize] as u16;
    let coarse = buffer[map.coarse as usize] as u16 * 255;
    fine + coarse
}
