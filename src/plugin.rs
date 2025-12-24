use bevy::prelude::*;

use crate::{
    assets::{HidAsset, HidAssetLoader, TemporaryDeviceAssets},
    device::DeviceAsset,
    resources::{HidApi, HumanInterfaceDevices},
    systems::*,
};

pub struct HidPlugin;

impl Plugin for HidPlugin {
    fn build(&self, app: &mut App) {
        match HidApi::new() {
            Ok(api) => {
                app.insert_resource(api)
                    .init_resource::<HumanInterfaceDevices>()
                    .register_type::<HumanInterfaceDevices>()
                    .register_type::<HidAsset>()
                    .init_asset::<DeviceAsset>()
                    .init_asset::<HidAsset>()
                    .register_asset_reflect::<HidAsset>()
                    .init_asset_loader::<HidAssetLoader>()
                    .add_systems(PreStartup, load_device_assets)
                    .add_systems(
                        PreUpdate,
                        (
                            check_device_assets_loaded
                                .run_if(resource_exists::<TemporaryDeviceAssets>),
                            update_hid_devices.run_if(resource_exists::<HumanInterfaceDevices>),
                        ),
                    );
            }
            Err(err) => error!("Failed to start Hid client: {}", err),
        }
    }
}
