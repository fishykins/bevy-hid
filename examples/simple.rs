use bevy::prelude::*;
use bevy_hid::{DeviceConfig, HidPlugin};
use hid_and_seek::HumanInterfaceDevice;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HidPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut device_maps: ResMut<Assets<DeviceConfig>>) {
    // Unlike Gilrs, Hid will only initialize devices that have a config file loaded as an asset.
    // This helps eliminate unwanted device clutter.
    let joystick = HumanInterfaceDevice::from_file("devices/nxt_gladiator.ron")
    .expect("couldn't create nxt device");

    assert_eq!(joystick.id.product_id, 512);
    assert_eq!(joystick.id.vendor_id, 8989);

    // Any devices that match this type will be automatically found and initialized.
    device_maps.add(DeviceConfig::new(joystick));
}
