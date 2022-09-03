use bevy::prelude::*;
use bevy_hid::{DeviceConfig, HidPlugin};
use hidasp::HumanInterfaceDevice;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HidPlugin)
        .add_startup_system(setup)
        .add_system(gamepad_events)
        .run();
}

fn setup(mut device_maps: ResMut<Assets<DeviceConfig>>) {
    // Unlike Gilrs, Hid will only initialize devices that have a config file loaded as an asset.
    // This helps eliminate unwanted device clutter.
    let joystick = HumanInterfaceDevice::from_file("hid_mappings/vendors/vkb/nxt_gladiator.ron")
    .expect("couldn't create nxt device");

    assert_eq!(joystick.device_type.product_id, 512);
    assert_eq!(joystick.device_type.vendor_id, 8989);

    // Any devices that match this type will be automatically found and initialized.
    device_maps.add(DeviceConfig::new(joystick));
}

fn gamepad_events(mut gamepad_event: EventReader<GamepadEvent>) {
    for event in gamepad_event.iter() {
        match event.event_type {
            GamepadEventType::Connected => {
                info!("{:?} Connected", event.gamepad);
            }
            GamepadEventType::Disconnected => {
                info!("{:?} Disconnected", event.gamepad);
            }
            GamepadEventType::ButtonChanged(button_type, value) => {
                info!(
                    "{:?} of {:?} is changed to {}",
                    button_type, event.gamepad, value
                );
            }
            GamepadEventType::AxisChanged(axis_type, value) => {
                info!(
                    "{:?} of {:?} is changed to {}",
                    axis_type, event.gamepad, value
                );
            }
        }
    }
}
