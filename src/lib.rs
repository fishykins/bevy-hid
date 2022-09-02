mod systems;
mod transcode;
mod manager;
use bevy::{input::InputSystem, prelude::*, reflect::TypeUuid, utils::HashMap};
use hid_and_seek::*;
use hidapi::HidApi;
use systems::*;

pub type DeviceBuffer = [u8; 256];

#[derive(Debug, Clone, Resource, Default)]
pub struct DeviceBuffers(HashMap<String, DeviceBuffer>);


#[derive(Debug, TypeUuid, Clone)]
#[uuid = "39cadf23-ab9d-2754-3142-d018b53b4257"]
pub struct DeviceConfig(HumanInterfaceDevice);

impl DeviceConfig {
    pub fn new(hid: HumanInterfaceDevice) -> Self {
        Self(hid)
    }

    pub fn id(&self) -> &DeviceUid {
        &self.0.id
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
                    .add_system_to_stage(
                        CoreStage::PreUpdate,
                        hid_event_system.before(InputSystem),
                    )
                    .add_system_to_stage(
                        CoreStage::PreUpdate,
                        hid_asset_update_system.before(InputSystem),
                    );
            }
            Err(err) => error!("Failed to start Hid. {}", err),
        }
    }
}