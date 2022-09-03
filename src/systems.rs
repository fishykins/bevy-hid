use crate::{transcode::transcode_device, DeviceBuffer, DeviceBuffers, HidConfigs};

use super::DeviceConfig;
use bevy::input::gamepad::GamepadEventRaw;
use bevy::prelude::*;
use hid_and_seek::{DeviceType, utils::build_buffer_map};
use hidapi::{DeviceInfo, HidApi};

pub fn hid_event_system(
    hid: NonSend<HidApi>,
    configs: Res<HidConfigs>,
    _events: EventWriter<GamepadEventRaw>,
    mut buffers: ResMut<DeviceBuffers>,
) {
    for device in hid.device_list() {
        let device_type = DeviceType {
            vendor_id: device.vendor_id(),
            product_id: device.product_id(),
        };

        if let Some((cfg, buf_map)) = configs.0.get(&device_type) {
            // we have a valid config for this device- attempt to open a steam
            if let Ok(stream) = device.open_device(&hid) {
                stream
                    .set_blocking_mode(false)
                    .expect("failed to unblock device");
                // We have a stream established, try and get buffers and send to transcode.
                let mut buf = [0u8; 256];
                let buf_new: Option<&DeviceBuffer> = match stream.read(&mut buf[..]) {
                    Ok(_) => Some(&buf),
                    Err(_) => None,
                };
                let path = String::from_utf8_lossy(device.path().to_bytes()).to_string();
                let buf_old = buffers.0.get(&path);
                let _transcoded = transcode_device(&cfg, buf_map, buf_new, buf_old);

                // Update the cache
                if valid_buffer(buf_new) {
                    buffers.0.insert(path, *buf_new.unwrap());
                }
            }
        }
    }
}

pub fn hid_asset_update_system(
    hid: NonSend<HidApi>,
    assets: Res<Assets<DeviceConfig>>,
    mut asset_events: EventReader<AssetEvent<DeviceConfig>>,
    mut gamepad_events: EventWriter<GamepadEventRaw>,
    mut handles: ResMut<HidConfigs>,
) {
    if asset_events.is_empty() {
        return;
    }

    for event in asset_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(asset) = assets.get(handle) {
                    for device in hid.device_list() {
                        let type_id = DeviceType {
                            vendor_id: device.vendor_id(),
                            product_id: device.product_id(),
                        };
                        if type_id == *asset.id() {
                            // A device has been found that matches this asset description.
                            // Send a connection event to bevy!
                            gamepad_events.send(GamepadEventRaw::new(
                                Gamepad {
                                    id: device.interface_number() as usize,
                                },
                                GamepadEventType::Connected,
                            ));
                            // As the asset is going to be dropped due to a lack of handles, 
                            // we need to move a copy of the config into our own storage resource.
                            let cfg = asset.inner().clone();
                            let buf_map = build_buffer_map(&cfg.map); 
                            handles.0.insert(type_id.clone(), (cfg, buf_map));
                        }
                    }
                }
            }
            AssetEvent::Modified { handle } => {
                if let Some(asset) = assets.get(handle) {
                    warn!("HID config modified: {}: {:?}", asset.name(), asset.id());
                }
            }
            AssetEvent::Removed { handle } => {
                if let Some(asset) = assets.get(handle) {
                    warn!("HID config removed: {}: {:?}", asset.name(), asset.id());
                    for device in hid.device_list() {
                        let type_id = DeviceType {
                            vendor_id: device.vendor_id(),
                            product_id: device.product_id(),
                        };
                        if type_id == *asset.id() {
                            // A device has been found that matches this asset description.
                            // Send a connection event to bevy!
                            gamepad_events.send(GamepadEventRaw::new(
                                Gamepad {
                                    id: get_device_id(device),
                                },
                                GamepadEventType::Disconnected,
                            ));
                        }
                    }
                }
            }
        }
    }
}

fn get_device_id(info: &DeviceInfo) -> usize {
    info.interface_number() as usize
}

pub fn valid_buffer(buf: Option<&DeviceBuffer>) -> bool {
    if let Some(b) = buf {
        return b[0] == 1
    }
    false
}