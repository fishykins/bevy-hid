use bevy::platform::cell::SyncCell;
use bevy::prelude::*;
use hidapi::HidResult;
use std::collections::HashMap;

use crate::{assets::HidAsset, device::DeviceId};

#[derive(Resource)]
pub(crate) struct HidApi {
    pub(crate) cell: SyncCell<hidapi::HidApi>,
}

#[derive(Clone, Resource, Default, Reflect)]
pub struct HumanInterfaceDevices {
    pub(crate) assets: HashMap<DeviceId, Handle<HidAsset>>,
    pub(crate) connected: HashMap<String, Entity>,
}

impl HidApi {
    pub(crate) fn new() -> HidResult<Self> {
        match hidapi::HidApi::new() {
            Ok(api) => Ok(Self {
                cell: SyncCell::new(api),
            }),
            Err(err) => {
                error!("Failed to start Hid client: {}", err);
                Err(err)
            }
        }
    }
}

impl HumanInterfaceDevices {
    pub(crate) fn new(devices: HashMap<DeviceId, Handle<HidAsset>>) -> Self {
        Self {
            assets: devices,
            connected: HashMap::new(),
        }
    }

    /// Gets a list of all the connected devices.
    pub fn devices(&self) -> Vec<Entity> {
        self.connected.values().cloned().collect()
    }
}
