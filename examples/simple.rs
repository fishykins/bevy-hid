use bevy_hid::buffer_map::BufferMap;
use hidapi::HidApi;
use std::{collections::HashMap, thread::sleep, time::Duration};

const SLEEP_TIME: u64 = 10;

fn main() {
    println!("Printing all available hid devices:");

    let device_map = load_device(8989, 512, "map_1").unwrap();
    let buffer_map = BufferMap::from(&device_map);

    println!("{:?}\n", device_map);
    println!("{:?}", buffer_map);

    // Map DeviceId -> (BufferMap, DeviceMap)
    let mut device_buffers = HashMap::new();
    // Map DeviceId -> Last Buffer State
    let mut device_states: HashMap<DeviceId, [u8; 256]> = HashMap::new();

    // Clone device_map if you were to use it elsewhere, but here we can just move it.
    // However, BufferMap::from(&device_map) borrows it.
    // We can clone device_map for the HashMap.
    device_buffers.insert(DeviceId::new(8989, 512), (buffer_map, device_map));

    let Ok(hid) = HidApi::new() else {
        panic!();
    };

    loop {
        for device in hid.device_list() {
            let id = DeviceId::from(device);

            let Some((buffer_map, device_map)) = device_buffers.get(&id) else {
                continue;
            };

            let Ok(stream) = device.open_device(&hid) else {
                continue;
            };

            stream
                .set_blocking_mode(false)
                .expect("failed to unblock device");

            // Get last known state or default to zeros
            let buf_last = *device_states.get(&id).unwrap_or(&[0u8; 256]);

            let mut buf = [0u8; 256];
            // Read up to 256 bytes
            let buf_new = match stream.read(&mut buf[..]) {
                Ok(n) if n > 0 => &buf,
                _ => {
                    continue;
                }
            };

            if buf_new == &buf_last {
                continue;
            }

            //println!("Device {:?}: {:?}", id, buf_new);
            //println!("Device {:?}: {:?}", id, buf_new[17]);

            for (i, binds) in buffer_map.iter() {
                let i = *i as usize;

                let last = buf_last[i];
                let new = buf_new[i];

                if last == new {
                    continue;
                }

                for bind in binds.iter() {
                    let Some(input) = device_map.get(bind) else {
                        continue;
                    };

                    match input {
                        InputType::Button(ptr) => {
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
                                    "Button {:?} {:?}: {}",
                                    bind,
                                    ptr,
                                    if pressed { "Pressed" } else { "Released" }
                                );
                            }
                        }
                        InputType::Axis(ptr) => {
                            // Assuming 8-bit axes for simplicity if octaves logic isn't clear,
                            // but implementation plan mentioned fine/coarse.
                            // Let's print the raw value for now or try to combine.
                            // If ptr.coarse is used, we need that value too.
                            // But we are only triggered if 'i' changed. 'i' could be fine or coarse.

                            let fine_val = buf_new[ptr.fine as usize];
                            let coarse_val = buf_new[ptr.coarse as usize];
                            let _val = ((coarse_val as u16) << 8) | (fine_val as u16);

                            //println!("Axis {:?}: {}", bind, val);
                        }
                        InputType::ButtonGroup(group) => {
                            for (_id, ptr) in group {
                                // Check if this button is the one related to index i?
                                // Or just check all buttons in the group?
                                // The bind triggered, so we check the group.
                                // To avoid spam, we should check if this specific button changed.
                                if ptr.0 as usize == i {
                                    let pressed = match ptr.1 {
                                        ButtonQuery::Bit(mask) => (new & mask) != 0,
                                        ButtonQuery::Eq(val) => new == val,
                                    };
                                    let was_pressed = match ptr.1 {
                                        ButtonQuery::Bit(mask) => (last & mask) != 0,
                                        ButtonQuery::Eq(val) => last == val,
                                    };
                                    if pressed != was_pressed {
                                        // println!(
                                        //     "Group Button {:?} {:?}: {}",
                                        //     id,
                                        //     ptr,
                                        //     if pressed { "Pressed" } else { "Released" }
                                        // );
                                    }
                                }
                            }
                        }
                        InputType::AxisGroup(group) => {
                            // Similar logic for AxisGroup
                            for (_id, ptr) in group {
                                if ptr.fine as usize == i || ptr.coarse as usize == i {
                                    let fine_val = buf_new[ptr.fine as usize];
                                    let coarse_val = buf_new[ptr.coarse as usize];
                                    let _val = ((coarse_val as u16) << 8) | (fine_val as u16);
                                    //println!("Group Axis {:?}: {}", id, val);
                                }
                            }
                        }
                    }
                }
            }
            // Update state
            device_states.insert(id, *buf_new);
        }
        print_state(&device_states);
        sleep(Duration::from_millis(SLEEP_TIME));
    }
}

fn print_state(device_states: &HashMap<DeviceId, [u8; 256]>) {
    // ANSI escape code to move cursor to home position (0,0)
    print!("\x1b[H");
    for (id, buf) in device_states {
        println!("Device {:?}:", id);
        for (i, byte) in buf.iter().enumerate() {
            if i == 0 {
                print!("   ");
            }
            print!("{:02x} ", byte);
            if (i + 1) % 16 == 0 {
                println!();
                if i < 255 {
                    print!("   ");
                }
            }
        }
        println!();
    }
}
