use std::collections::{HashMap, HashSet};

use bevy::reflect::Reflect;

use crate::{bindings::Bind, device::DeviceMap, input::InputType};

/// A buffer map is essentially an inverted DeviceMap which indexes by buffer indices rather than action types.
/// This is not a good way to store data, but is very useful for quickly parsing device buffers.
#[derive(Debug, Clone, Reflect)]
pub struct BufferMap(HashMap<u8, Vec<Bind>>);

impl BufferMap {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, u8, Vec<Bind>> {
        self.0.iter()
    }
}

impl From<&DeviceMap> for BufferMap {
    fn from(device_map: &DeviceMap) -> Self {
        let mut buffer_map: HashMap<u8, HashSet<Bind>> = HashMap::new();

        // lets just get right to it
        for (bind, input) in device_map.iter() {
            //println!("  mappng {:?}...", bind);
            match input {
                InputType::Button(button) => {
                    if let Some(buf) = buffer_map.get_mut(&button.0) {
                        buf.insert(bind.clone());
                        //println!("    added to exisitng buf {}:{:?}", button.0, buf);
                    } else {
                        let mut hashset = HashSet::new();
                        hashset.insert(bind.clone());
                        println!("    added new buf {}:{:?}", button.0, hashset);
                        buffer_map.insert(button.0, hashset);
                    }
                }
                InputType::Axis(axis) => {
                    if let Some(buf) = buffer_map.get_mut(&axis.fine) {
                        buf.insert(bind.clone());
                    } else {
                        let mut hashset = HashSet::new();
                        hashset.insert(bind.clone());
                        buffer_map.insert(axis.fine, hashset);
                    }
                    if let Some(buf) = buffer_map.get_mut(&axis.coarse) {
                        buf.insert(bind.clone());
                    } else {
                        let mut hashset = HashSet::new();
                        hashset.insert(bind.clone());
                        buffer_map.insert(axis.coarse, hashset);
                    }
                }
                InputType::ButtonGroup(group) => {
                    for (_, button) in group {
                        if let Some(buf) = buffer_map.get_mut(&button.0) {
                            buf.insert(bind.clone());
                        } else {
                            let mut hashset = HashSet::new();
                            hashset.insert(bind.clone());
                            buffer_map.insert(button.0, hashset);
                        }
                    }
                }
                InputType::AxisGroup(group) => {
                    for (_, axis) in group {
                        if let Some(buf) = buffer_map.get_mut(&axis.fine) {
                            buf.insert(bind.clone());
                        } else {
                            let mut hashset = HashSet::new();
                            hashset.insert(bind.clone());
                            buffer_map.insert(axis.fine, hashset);
                        }
                        if let Some(buf) = buffer_map.get_mut(&axis.coarse) {
                            buf.insert(bind.clone());
                        } else {
                            let mut hashset = HashSet::new();
                            hashset.insert(bind.clone());
                            buffer_map.insert(axis.coarse, hashset);
                        }
                    }
                }
            }
        }
        BufferMap(
            buffer_map
                .iter()
                .map(|(buf, v)| (*buf, Vec::from_iter(v.clone())))
                .collect(),
        )
    }
}
