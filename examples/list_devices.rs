//! # list_devices Example
//!
//! Rust example that lists the available interfaces as
//! well as some basic information about those interfaces.
//!
//! This example is based off an official example written in Python available here:
//! https://gitlab.freedesktop.org/NetworkManager/NetworkManager/-/blob/main/examples/python/dbus/list-devices.py
//!
//! DISCLAIMER:
//! The example code provided here is for illustrative purposes only. It is provided "AS IS",
//! without warranty of any kind, express or implied, including but not limited to the warranties of
//! merchantability, fitness for a particular purpose, and non-infringement. In no event shall the authors
//! or copyright holders be liable for any claim, damages, or other liability, whether in an action of
//! contract, tort, or otherwise, arising from, out of, or in connection with the example code or
//! the use or other dealings in the example code.

use rusty_network_manager::{DeviceProxy, IP4ConfigProxy, NetworkManagerProxy};
use std::collections::HashMap;
use zbus::Connection;

#[tokio::main]
async fn main() {
    let connection = Connection::system()
        .await
        .expect("Could not get a connection.");

    let nm = NetworkManagerProxy::new(&connection)
        .await
        .expect("Could not get NetworkManager");

    let devtypes = get_devtypes();
    let states = get_states();

    for device in nm.get_all_devices().await.expect("Could not find devices") {
        let device_proxy = DeviceProxy::new_from_path(device, &connection)
            .await
            .expect("Error");

        println!("============================");
        println!("Interface: {}", device_proxy.interface().await.unwrap());

        println!(
            "Ip 4 Address: {}",
            get_ip4_address(&device_proxy, &connection).await
        );
        println!("Ip 4 config: {}", device_proxy.ip4_config().await.unwrap());

        let devtype_id = device_proxy.device_type().await.unwrap();
        match devtypes.get(&devtype_id) {
            Some(devtype) => println!("Type: {}", devtype),
            None => println!("Type: Unknown"),
        };

        let state_id = device_proxy.state().await.unwrap();
        match states.get(&state_id) {
            Some(state) => println!("State: {}", state),
            None => println!("Type: Unknown"),
        };
    }
}

async fn get_ip4_address(device_proxy: &DeviceProxy<'_>, connection: &Connection) -> String {
    let ip4config_path = device_proxy.ip4_config().await;
    let ip4config = IP4ConfigProxy::new_from_path(ip4config_path.unwrap(), connection).await;

    let Ok(config) = ip4config else {
        return String::from("Unknown");
    }; // Assuming ip4config is a Result type
    let Ok(address_data) = config.address_data().await else {
        return String::from("Unknown");
    };
    let Some(address) = address_data.first().and_then(|addr| addr.get("address")) else {
        return String::from("Unknown");
    };

    address.downcast_ref().unwrap()
}

fn get_states() -> HashMap<u32, String> {
    let mut states: HashMap<u32, String> = HashMap::new();

    states.insert(0, "Unknown".to_string());
    states.insert(10, "Unmanaged".to_string());
    states.insert(20, "Unavailable".to_string());
    states.insert(30, "Disconnected".to_string());
    states.insert(40, "Prepare".to_string());
    states.insert(50, "Config".to_string());
    states.insert(60, "Need Auth".to_string());
    states.insert(70, "IP Config".to_string());
    states.insert(80, "IP Check".to_string());
    states.insert(90, "Secondaries".to_string());
    states.insert(100, "Activated".to_string());
    states.insert(110, "Deactivating".to_string());
    states.insert(120, "Failed".to_string());

    states
}

fn get_devtypes() -> HashMap<u32, String> {
    let mut devtypes: HashMap<u32, String> = HashMap::new();

    devtypes.insert(1, "Ethernet".to_string());
    devtypes.insert(2, "Wi-Fi".to_string());
    devtypes.insert(5, "Bluetooth".to_string());
    devtypes.insert(6, "OLPC".to_string());
    devtypes.insert(7, "WiMAX".to_string());
    devtypes.insert(8, "Modem".to_string());
    devtypes.insert(9, "InfiniBand".to_string());
    devtypes.insert(10, "Bond".to_string());
    devtypes.insert(11, "VLAN".to_string());
    devtypes.insert(12, "ADSL".to_string());
    devtypes.insert(13, "Bridge".to_string());
    devtypes.insert(14, "Generic".to_string());
    devtypes.insert(15, "Team".to_string());
    devtypes.insert(16, "TUN".to_string());
    devtypes.insert(17, "IPTunnel".to_string());
    devtypes.insert(18, "MACVLAN".to_string());
    devtypes.insert(19, "VXLAN".to_string());
    devtypes.insert(20, "Veth".to_string());

    devtypes
}
