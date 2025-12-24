use std::collections::{HashMap, HashSet};

use bevy::{ecs::component::Component, prelude::GamepadButton, reflect::Reflect};

use crate::device::DeviceMap;

/// Stores the last buffer state of a hid. Used for internal caching.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
pub struct HidBuffer(pub(crate) [u8; 256]);

/// A buffer map is essentially an inverted DeviceMap which indexes by buffer indices rather than action types.
/// This minimizes buffer lookups and allows for quick parsing of device buffers.
#[derive(Debug, Clone, Reflect)]
pub struct BufferMap(HashMap<u8, Vec<GamepadButton>>);

impl Default for HidBuffer {
    fn default() -> Self {
        Self([0; 256])
    }
}

impl BufferMap {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, u8, Vec<GamepadButton>> {
        self.0.iter()
    }
}

impl From<&DeviceMap> for BufferMap {
    fn from(device_map: &DeviceMap) -> Self {
        let mut buffer_map: HashMap<u8, HashSet<GamepadButton>> = HashMap::new();

        // only buttons
        for (button, ptr) in &device_map.buttons {
            if let Some(buf) = buffer_map.get_mut(&ptr.0) {
                buf.insert(*button);
            } else {
                let mut hashset = HashSet::new();
                hashset.insert(*button);
                buffer_map.insert(ptr.0, hashset);
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
