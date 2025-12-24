use std::collections::HashMap;

use crate::{
    assets::{HidAsset, TemporaryDeviceAssets}, buffers::{BufferMap, HidBuffer}, device::{DeviceAsset, DeviceId}, input::ButtonQuery, resources::{HidApi, HumanInterfaceDevices}
};
use bevy::{
    asset::LoadedFolder,
    input::gamepad::{
        GamepadConnection, GamepadConnectionEvent, RawGamepadAxisChangedEvent,
        RawGamepadButtonChangedEvent, RawGamepadEvent,
    },
    prelude::*,
};

/// Starts the initial loading sequence.
pub(crate) fn load_device_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let device_assets = asset_server.load_folder("hid");
    commands.insert_resource(TemporaryDeviceAssets { device_assets });
}

/// Checks to see if all crate assets have been loaded and updates accordingly.
pub(crate) fn check_device_assets_loaded(
    mut commands: Commands,
    folder_handle: Option<Res<TemporaryDeviceAssets>>,
    device_assets: Res<Assets<DeviceAsset>>,
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
            let typed_handle: Handle<DeviceAsset> = handle.clone().typed();
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
    mut gamepads: Query<&mut HidBuffer, With<Gamepad>>,
    mut events: MessageWriter<RawGamepadEvent>,
    mut connection_events: MessageWriter<GamepadConnectionEvent>,
    mut button_events: MessageWriter<RawGamepadButtonChangedEvent>,
    mut axis_event: MessageWriter<RawGamepadAxisChangedEvent>,
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
            let gamepad = commands.spawn(HidBuffer::default()).id();

            let event = GamepadConnectionEvent::new(
                gamepad,
                GamepadConnection::Connected {
                    name: asset.name().to_string(),
                    vendor_id: Some(device.vendor_id()),
                    product_id: Some(device.product_id()),
                },
            );
            events.write(event.clone().into());
            connection_events.write(event);
            resources.connected.insert(path, gamepad);

            continue;
        }

        let Some(gamepad) = resources.connected.get(&path) else {
            continue;
        };
        let gamepad = *gamepad;

        let Ok(mut buffer_component) = gamepads.get_mut(gamepad) else {
            continue;
        };

        let Ok(stream) = device.open_device(&api) else {
            continue;
        };

        stream
            .set_blocking_mode(false)
            .expect("failed to unblock device");

        // Get last known state or default to zeros
        let mut buf = [0u8; 256];

        let buf_last = buffer_component.0.clone();
        let buf_new = match stream.read(&mut buf[..]) {
            Ok(n) if n > 0 => &mut buf,
            _ => {
                continue;
            }
        };
        buffer_component.0 = *buf_new;


        for (i, binds) in asset.buffer_map.iter() {
            let i = *i as usize;

            let last = buf_last[i];
            let new = buf_new[i];

            if last == new {
                continue;
            }

            for bind in binds.iter() {
                let bevy_gamepad_code: GamepadButton = match bind.get_type() {
                    crate::bindings::BindType::Button => {
                        match bind.get_id() {
                            crate::bindings::BindId::Id(id) => GamepadButton::Other(id as u8),
                            _ => continue,
                        }
                    }
                    _ => continue,
                };

                let Some(input) = asset.input_mapping.get(bind) else {
                    continue;
                };

                match input {
                    crate::input::InputType::Button(ptr) => {
                        let pressed = match ptr.1 {
                            ButtonQuery::Bit(mask) => (new & mask) != 0,
                            ButtonQuery::Eq(val) => new == val,
                        };
                        let was_pressed = match ptr.1 {
                            ButtonQuery::Bit(mask) => (last & mask) != 0,
                            ButtonQuery::Eq(val) => last == val,
                        };
                        if pressed != was_pressed {
                            let event = RawGamepadButtonChangedEvent::new(
                                gamepad,
                                bevy_gamepad_code,
                                if pressed { 1.0 } else { 0.0 },
                            );
                            events.write(event.clone().into());
                            button_events.write(event);
                        }
                    }
                    _ => {}
                }
            }
        }


        let axis = GamepadAxis::Other(1);
        let raw_value = 0.42;
        //
        events.write(RawGamepadAxisChangedEvent::new(gamepad, axis, raw_value).into());
        axis_event.write(RawGamepadAxisChangedEvent::new(gamepad, axis, raw_value));
    }
}
