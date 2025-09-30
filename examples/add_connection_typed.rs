//! # add_connection_typed Example
//!
//! Rust example that demonstrates creating a WiFi connection using the new typed API.
//! This example shows both the old HashMap-based approach and the new typed approach.
//!
//! Usage: ./add_connection_typed <connection_name> <ssid> <password>
//! Example: ./add_connection_typed MyWiFiConnection MySSID MyPassword
//!
//! DISCLAIMER:
//! The example code provided here is for illustrative purposes only. It is provided "AS IS",
//! without warranty of any kind, express or implied, including but not limited to the warranties of
//! merchantability, fitness for a particular purpose, and non-infringement. In no event shall the authors
//! or copyright holders be liable for any claim, damages, or other liability, whether in an action of
//! contract, tort, or otherwise, arising from, out of, or in connection with the example code or
//! the use or other dealings in the example code.

use rusty_network_manager::{NetworkManagerProxy, DeviceProxy, DeviceType, ConnectionProfile};
use std::env;
use zbus::Connection;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: ./add_connection_typed <connection_name> <ssid> <password>");
        eprintln!("Example: ./add_connection_typed MyWiFiConnection MySSID MyPassword");
        std::process::exit(1);
    }

    let connection_name = &args[1];
    let ssid = &args[2];
    let password = &args[3];

    let connection = Connection::system()
        .await
        .expect("Could not get system connection");

    let nm = NetworkManagerProxy::new(&connection)
        .await
        .expect("Could not get NetworkManager");

    // Find a Wi-Fi device
    let wireless_device = find_wireless_device(&nm, &connection).await
        .expect("Could not find a wireless device");

    println!("Found wireless device: {}", wireless_device);

    // Create a connection profile using the new typed API
    let profile = ConnectionProfile::new_wifi_wpa2(
        connection_name.clone(),
        ssid.clone(),
        password.clone(),
    );

    println!("Creating connection profile '{}' for SSID '{}'", connection_name, ssid);

    let specific_object = zbus::zvariant::OwnedObjectPath::try_from("/").unwrap();

    match nm.add_and_activate_connection_typed(
        profile,
        &wireless_device,
        &specific_object,
    ).await {
        Ok((active_connection_path, connection_path)) => {
            println!("✓ Successfully created and activated connection!");
            println!("  Active connection: {}", active_connection_path);
            println!("  Connection path: {}", connection_path);
        }
        Err(e) => {
            eprintln!("✗ Failed to create connection: {}", e);
            std::process::exit(1);
        }
    }
}

async fn find_wireless_device(
    nm: &NetworkManagerProxy<'_>,
    connection: &Connection,
) -> Result<zbus::zvariant::OwnedObjectPath, Box<dyn std::error::Error>> {
    let devices = nm.get_all_devices().await?;

    for device_path in devices {
        let device_proxy = DeviceProxy::new_from_path(device_path.clone(), connection).await?;
        
        let device_type = device_proxy.get_device_type().await?;
        
        if device_type == DeviceType::Wifi {
            let interface = device_proxy.interface().await?;
            println!("Found Wi-Fi device: {} ({})", interface, device_path);
            return Ok(device_path);
        }
    }

    Err("No Wi-Fi device found".into())
}