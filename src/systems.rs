use std::collections::HashMap;

use crate::{
    assets::{HidAsset, TemporaryDeviceAssets},
    buffer_map::BufferMap,
    device::{DeviceId, HumanInterfaceDevice},
    resources::{HidApi, HumanInterfaceDevices},
};
use bevy::{asset::LoadedFolder, prelude::*};

/// Starts the initial loading sequence.
pub(crate) fn load_device_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let device_assets = asset_server.load_folder("hid");
    commands.insert_resource(TemporaryDeviceAssets { device_assets });
}

/// Checks to see if all crate assets have been loaded and updates accordingly.
pub(crate) fn check_device_assets_loaded(
    mut commands: Commands,
    folder_handle: Option<Res<TemporaryDeviceAssets>>,
    device_assets: Res<Assets<HumanInterfaceDevice>>,
    mut mapped_assets: ResMut<Assets<HidAsset>>,
    loaded_folders: Res<Assets<LoadedFolder>>,
) {
    let Some(folder_handle) = folder_handle else {
        return;
    };

    if let Some(folder) = loaded_folders.get(&folder_handle.device_assets)
        && !device_assets.is_empty()
    {
        info!(
            "Ship classes folder loaded, containing {} assets.",
            folder.handles.len()
        );

        let mut devices = HashMap::new();

        for handle in &folder.handles {
            let typed_handle: Handle<HumanInterfaceDevice> = handle.clone().typed();
            if let Some(device) = device_assets.get(&typed_handle) {
                let new_handle = mapped_assets.add(HidAsset {
                    name: device.name().to_string(),
                    input_mapping: device.mappings().clone(),
                    buffer_map: BufferMap::from(device.mappings()),
                });
                devices.insert(DeviceId::from(device), new_handle);
            }
        }

        commands.insert_resource(HumanInterfaceDevices::new(devices));
        commands.remove_resource::<TemporaryDeviceAssets>();
    }
}

pub(crate) fn update_hid_devices(
    mut commands: Commands,
    mut hid: ResMut<HidApi>,
    mut resources: ResMut<HumanInterfaceDevices>,
    assets: Res<Assets<HidAsset>>,
) {
    let api = hid.cell.get();

    let devices = api.device_list();
    for device in devices {
        let path = device.path().to_string_lossy().to_string();

        let id = DeviceId::from(device);

        let Some(handle) = resources.assets.get(&id) else {
            continue;
        };

        let Some(asset) = assets.get(handle) else {
            continue;
        };

        if !resources.connected.contains_key(&path) {
            // New device- initiate it.
            // HELP! Gamepad is private and we cant add custom mappings to it (yet).
            // GO FIX BEVY_INPUT!
            let entity = commands.spawn(Gamepad::default()).id();

            resources.connected.insert(path, id);
            info!("Device {:?} is connected.", asset.name());
            // For the moment, continue. This device will be updated in the next frame.
            continue;
        }
    }
}
