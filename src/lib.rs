mod systems;
mod transcode;
pub(crate) mod event_mapping;

use bevy::{input::InputSystem, prelude::*, reflect::TypeUuid, utils::HashMap};
use hidasp::{*, maps::BufferMap};
use hidapi::HidApi;
use systems::*;

pub type DeviceBuffer = [u8; 256];

#[derive(Debug, Clone, Resource, Default)]
pub struct DeviceBuffers(HashMap<String, DeviceBuffer>);

#[derive(Debug, Clone, Resource, Default)]

/// A HashMap linking device ids to their respective asset.
pub struct HidConfigs(HashMap<DeviceType, (HumanInterfaceDevice, BufferMap)>);

/// This is a wrapper for a `HumanInterfaceDevice`, and is only used to get configs in to the engine via Assets.
/// Once it is available to bevy, it will be consumed and converted back into it's regular configuration. 
#[derive(Debug, TypeUuid, Clone)]
#[uuid = "39cadf23-ab9d-2754-3142-d018b53b4257"]
pub struct DeviceConfig(HumanInterfaceDevice);

impl DeviceConfig {
    pub fn new(hid: HumanInterfaceDevice) -> Self {
        Self(hid)
    }

    pub fn id(&self) -> &DeviceType {
        &self.0.device_type
    }

    pub fn name(&self) -> &str {
        &self.0.name
    }

    pub fn inner(&self) -> &HumanInterfaceDevice {
        &self.0
    }
}

#[derive(Default)]
pub struct HidPlugin;

impl Plugin for HidPlugin {
    fn build(&self, app: &mut App) {
        match HidApi::new() {
            Ok(api) => {
                app.insert_non_send_resource(api)
                    .add_asset::<DeviceConfig>()
                    .init_resource::<DeviceBuffers>()
                    .init_resource::<HidConfigs>()
                    .add_system_to_stage(CoreStage::PreUpdate, hid_event_system.before(InputSystem))
                    .add_system_to_stage(
                        CoreStage::PreUpdate,
                        hid_asset_update_system.before(InputSystem),
                    );
            }
            Err(err) => error!("Failed to start Hid. {}", err),
        }
    }
}
