use crate::input::{AxisPointer, ButtonPointer};
use bevy::{
    asset::Asset,
    prelude::{GamepadAxis, GamepadButton},
    reflect::Reflect,
};
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
#[derive(Debug, Clone, PartialEq, Reflect, Deserialize)]
pub struct DeviceMap {
    pub buttons: HashMap<GamepadButton, ButtonPointer>,
    pub axes: HashMap<GamepadAxis, AxisPointer>,
}

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
    pub fn new(
        buttons: HashMap<GamepadButton, ButtonPointer>,
        axes: HashMap<GamepadAxis, AxisPointer>,
    ) -> Self {
        Self { buttons, axes }
    }

    pub fn len(&self) -> usize {
        self.buttons.len() + self.axes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buttons.is_empty() && self.axes.is_empty()
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
