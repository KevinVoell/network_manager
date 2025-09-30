//! # list_devices Example
//!
//! Rust example that lists the available interfaces as
//! well as some basic information about those interfaces.
//! This example demonstrates the use of typed enums for device properties.
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

use rusty_network_manager::{
    DeviceProxy, DeviceState, DeviceCapabilities, Metered, ConnectivityState,
    DeviceInterfaceFlags, IP4ConfigProxy, NetworkManagerProxy
};
use zbus::Connection;

#[tokio::main]
async fn main() {
    let connection = Connection::system()
        .await
        .expect("Could not get a connection.");

    let nm = NetworkManagerProxy::new(&connection)
        .await
        .expect("Could not get NetworkManager");

    for device in nm.get_all_devices().await.expect("Could not find devices") {
        let device_proxy = DeviceProxy::new_from_path(device, &connection)
            .await
            .expect("Error");

        println!("============================");
        println!("Interface: {}", device_proxy.interface().await.unwrap());

        // Use typed methods instead of raw u32 values
        let device_type = device_proxy.get_device_type().await.unwrap();
        let device_state = device_proxy.get_state().await.unwrap();
        let capabilities = device_proxy.get_capabilities().await.unwrap();
        let metered = device_proxy.get_metered().await.unwrap();
        let interface_flags = device_proxy.get_interface_flags().await.unwrap();

        println!("Type: {}", device_type);
        println!("State: {}", device_state);
        
        // Show additional typed information
        println!("Capabilities: {}", capabilities);
        println!("Metered: {}", metered);
        println!("Interface Flags: {}", interface_flags);

        // Get connectivity information
        let ip4_connectivity = device_proxy.get_ip4_connectivity().await.unwrap();
        let ip6_connectivity = device_proxy.get_ip6_connectivity().await.unwrap();
        println!("IPv4 Connectivity: {}", ip4_connectivity);
        println!("IPv6 Connectivity: {}", ip6_connectivity);

        // Get state reason
        let (state, reason) = device_proxy.get_state_reason().await.unwrap();
        println!("State Reason: {} ({})", reason, state);

        // Show some helpful status checks using the new helper methods
        print_device_status(&device_state, &capabilities, &metered, &interface_flags, &ip4_connectivity);

        println!(
            "IPv4 Address: {}",
            get_ip4_address(&device_proxy, &connection).await
        );
        println!("IPv4 Config Path: {}", device_proxy.ip4_config().await.unwrap());
    }
}

fn print_device_status(
    state: &DeviceState,
    capabilities: &DeviceCapabilities,
    metered: &Metered,
    flags: &DeviceInterfaceFlags,
    connectivity: &ConnectivityState,
) {
    println!("--- Device Status ---");
    
    // Check state
    match state {
        DeviceState::Activated => println!("✓ Device is active and connected"),
        DeviceState::Disconnected => println!("✗ Device is disconnected"),
        DeviceState::Failed => println!("✗ Device connection failed"),
        DeviceState::Unavailable => println!("⚠ Device is unavailable"),
        _ => println!("ℹ Device state: {}", state),
    }

    // Check capabilities
    if capabilities.contains(DeviceCapabilities::NM_SUPPORTED) {
        println!("✓ Supported by NetworkManager");
    }
    if capabilities.contains(DeviceCapabilities::CARRIER_DETECT) {
        println!("✓ Can detect carrier status");
    }
    if capabilities.contains(DeviceCapabilities::IS_SOFTWARE) {
        println!("ℹ Software device");
    }

    // Check metered status
    if metered.is_metered() {
        println!("💰 Connection is metered (limited data)");
    } else if metered.is_not_metered() {
        println!("✓ Connection is not metered");
    }

    // Check interface status
    if flags.is_operational() {
        println!("✓ Interface is fully operational");
    } else {
        if !flags.is_up() {
            println!("✗ Interface is down");
        }
        if !flags.has_carrier() {
            println!("✗ No carrier detected");
        }
    }

    // Check connectivity
    if connectivity.has_full_connectivity() {
        println!("🌐 Full internet connectivity");
    } else if connectivity.has_connectivity() {
        println!("🔒 Limited connectivity");
    } else if connectivity.has_captive_portal() {
        println!("🚪 Captive portal detected");
    } else if connectivity.has_no_connectivity() {
        println!("✗ No connectivity");
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
