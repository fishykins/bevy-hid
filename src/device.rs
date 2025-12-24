use crate::{bindings::Bind, input::InputType};
use bevy::{asset::Asset, reflect::Reflect};
use hidapi::DeviceInfo;
use serde::Deserialize;
use std::collections::HashMap;

/// A readable asset from file.
#[derive(Clone, Debug, Asset, Deserialize, Reflect)]
pub struct DeviceAsset {
    name: String,
    pid: u16,
    vid: u16,
    mappings: DeviceMap,
}

/// This is where all data pertaining to a device is held.
#[derive(Debug, Clone, PartialEq, Deserialize, Reflect)]
pub struct DeviceMap(HashMap<Bind, InputType>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub struct DeviceId {
    pub vendor_id: u16,
    pub product_id: u16,
}

impl DeviceId {
    pub fn new(vendor_id: u16, product_id: u16) -> Self {
        Self {
            vendor_id,
            product_id,
        }
    }
}

impl DeviceAsset {
    pub fn new(name: String, pid: u16, vid: u16, mappings: DeviceMap) -> DeviceAsset {
        DeviceAsset {
            name,
            pid,
            vid,
            mappings,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    // Product ID
    pub fn pid(&self) -> u16 {
        self.pid
    }

    /// Vendor ID
    pub fn vid(&self) -> u16 {
        self.vid
    }

    pub fn mappings(&self) -> &DeviceMap {
        &self.mappings
    }
}

impl DeviceMap {
    pub fn new(map: HashMap<Bind, InputType>) -> Self {
        Self(map)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, Bind, InputType> {
        self.0.iter()
    }

    pub fn get(&self, bind: &Bind) -> Option<&InputType> {
        self.0.get(bind)
    }
}

impl IntoIterator for DeviceMap {
    type Item = (Bind, InputType);
    type IntoIter = std::collections::hash_map::IntoIter<Bind, InputType>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl From<&DeviceInfo> for DeviceId {
    fn from(device: &DeviceInfo) -> Self {
        Self {
            vendor_id: device.vendor_id(),
            product_id: device.product_id(),
        }
    }
}

impl From<&DeviceAsset> for DeviceId {
    fn from(device: &DeviceAsset) -> Self {
        Self {
            vendor_id: device.vid(),
            product_id: device.pid(),
        }
    }
}
