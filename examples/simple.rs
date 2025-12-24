use bevy_hid::{
    buffers::BufferMap,
    device::{DeviceAsset, DeviceId, DeviceMap},
    input::ButtonQuery,
};
use hidapi::HidApi;
use ron::de::from_reader;
use std::{collections::HashMap, fs::File, thread::sleep, time::Duration};

const SLEEP_TIME: u64 = 10;

fn load_device(path: &str) -> Option<DeviceMap> {
    let f = File::open(path).ok()?;
    let asset: DeviceAsset = from_reader(f).ok()?;
    Some(asset.mappings().clone())
}

fn main() {
    println!("Printing all available hid devices:");

    // Adjust path as needed, assuming running from crate root
    // In a real app the AssetServer handles this, but here we load manually.
    let device_map = load_device("assets/hid/nxt_gladiator.hid.ron").expect("Failed to load asset");
    let buffer_map = BufferMap::from(&device_map);

    println!(
        "Loaded Device Map with {} buttons and {} axes",
        device_map.buttons.len(),
        device_map.axes.len()
    );

    // Map DeviceId -> (BufferMap, DeviceMap)
    let mut device_search = HashMap::new();
    // Use the PID/VID from the asset/ron (8989, 512 for NXT Gladiator)
    device_search.insert(DeviceId::new(8989, 512), (buffer_map, device_map));

    // Map Connected DeviceId -> Last Buffer State
    let mut device_states: HashMap<DeviceId, [u8; 256]> = HashMap::new();

    let Ok(hid) = HidApi::new() else {
        panic!("Failed to init HidApi");
    };

    println!("Listening for devices...");

    loop {
        for device in hid.device_list() {
            let id = DeviceId::from(device);

            let Some((buffer_map, device_map)) = device_search.get(&id) else {
                continue;
            };

            // Only open if not already open? simple.rs just opens every loop if it can?
            // Actually typical hidapi usage: open, keep open.
            // But this example loop structure suggests it tries to open if matched.

            let Ok(stream) = device.open_device(&hid) else {
                continue;
            };

            let _ = stream.set_blocking_mode(false);

            let mut buf = [0u8; 256];
            // Read
            let buf_new = match stream.read(&mut buf[..]) {
                Ok(n) if n > 0 => &buf,
                _ => continue,
            };

            let buf_last = *device_states.get(&id).unwrap_or(&[0u8; 256]);

            if buf_new == &buf_last {
                continue;
            }

            // 1. Process Buttons using BufferMap
            for (i, buttons) in buffer_map.iter() {
                let i = *i as usize;

                let last = buf_last[i];
                let new = buf_new[i];

                if last == new {
                    continue;
                }

                for button in buttons.iter() {
                    if let Some(ptr) = device_map.buttons.get(button) {
                        let pressed = match ptr.1 {
                            ButtonQuery::Bit(mask) => (new & mask) != 0,
                            ButtonQuery::Eq(val) => new == val,
                        };
                        let was_pressed = match ptr.1 {
                            ButtonQuery::Bit(mask) => (last & mask) != 0,
                            ButtonQuery::Eq(val) => last == val,
                        };
                        if pressed != was_pressed {
                            println!(
                                "Button {:?} ({:?}): {}",
                                button,
                                ptr,
                                if pressed { "Pressed" } else { "Released" }
                            );
                        }
                    }
                }
            }

            // 2. Process Axes
            for (axis, ptr) in &device_map.axes {
                let fine_val = buf_new[ptr.fine as usize];
                let coarse_val = buf_new[ptr.coarse as usize];

                let fine_last = buf_last[ptr.fine as usize];
                let coarse_last = buf_last[ptr.coarse as usize];

                if fine_val == fine_last && coarse_val == coarse_last {
                    continue;
                }

                let val = ((coarse_val as u16) << 8) | (fine_val as u16);
                println!("Axis {:?}: {}", axis, val);
            }

            device_states.insert(id, *buf_new);
        }
        sleep(Duration::from_millis(SLEEP_TIME));
    }
}
