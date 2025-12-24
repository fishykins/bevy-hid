use crate::device::DeviceAsset;
use bevy::prelude::*;
use std::fs::read_dir;

/// Loads devices from disk.
pub fn load_raw_devices() -> Vec<DeviceAsset> {
    let temp_devices_path = format!("{}/assets/hid", env!("CARGO_MANIFEST_DIR"));
    let devices = read_dir(temp_devices_path).expect("devices directory not found");

    let mut all_devices = Vec::new();

    for device in devices {
        let device = device.expect("failed to read device");
        let path = device.path();

        let content = std::fs::read_to_string(path).expect("failed to read device file");
        let hid_device: DeviceAsset = match ron::from_str(&content) {
            Ok(device) => device,
            Err(e) => {
                println!("failed to deserialize device: {}", e);
                continue;
            }
        };

        all_devices.push(hid_device);
    }
    all_devices
}

// test modules
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_raw_devices() {
        let devices = load_raw_devices();
        assert!(!devices.is_empty());
    }
}
