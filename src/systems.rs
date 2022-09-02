use crate::{DeviceBuffer, DeviceBuffers, transcode::transcode_device};

use super::DeviceConfig;
use bevy::input::gamepad::GamepadEventRaw;
use bevy::prelude::*;
use bevy::utils::HashMap;
use hid_and_seek::DeviceUid;
use hidapi::{DeviceInfo, HidApi};

#[derive(Debug, Clone, Resource)]

/// A HashMap linking device ids to their respective asset.
pub struct HidConfigs(HashMap<DeviceUid, Handle<DeviceConfig>>);

pub fn hid_event_system(
    hid: NonSend<HidApi>,
    assets: Res<Assets<DeviceConfig>>,
    handles: Res<HidConfigs>,
    _events: EventWriter<GamepadEventRaw>,
    mut buffers: ResMut<DeviceBuffers>,
) {
    for device in hid.device_list() {
        let type_id = DeviceUid {
            vendor_id: device.vendor_id(),
            product_id: device.product_id(),
        };
        if let Some(handle) = handles.0.get(&type_id) {
            if let Some(config) = assets.get(handle) {
                // we have a valid config for this device- attempt to open a steam
                if let Ok(stream) = device.open_device(&hid) {
                    // We have a stream established, try and get buffers and send to transcode.
                    let mut buf = [0u8; 256];
                    let buf_new: Option<&DeviceBuffer> = match stream.read(&mut buf[..]) {
                        Ok(_) => Some(&buf),
                        Err(_) => None,
                    };
                    let path = String::from_utf8_lossy(device.path().to_bytes()).to_string();
                    let buf_old = buffers.0.get(&path);
                    let transcoded = transcode_device(config.inner(), buf_new, buf_old);

                    // Update the cache
                    if let Some(buf) = buf_new {
                        buffers.0.insert(path, *buf);
                    } else {
                        buffers.0.remove(&path);
                    }
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
                        let type_id = DeviceUid {
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
                            handles.0.insert(type_id.clone(), handle.clone());
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
                    for device in hid.device_list() {
                        let type_id = DeviceUid {
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
