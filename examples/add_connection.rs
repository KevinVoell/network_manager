//! # add_connection Example
//!
//! Rust example that creates a connection profile and activates it.
//! Usage: ./add_connection <ifname>
//!
//! Example: ./add_connection MyConnectionName MySSID MyPassword
//!
//! DISCLAIMER:
//! The example code provided here is for illustrative purposes only. It is provided "AS IS",
//! without warranty of any kind, express or implied, including but not limited to the warranties of
//! merchantability, fitness for a particular purpose, and non-infringement. In no event shall the authors
//! or copyright holders be liable for any claim, damages, or other liability, whether in an action of
//! contract, tort, or otherwise, arising from, out of, or in connection with the example code or
//! the use or other dealings in the example code.

use rusty_network_manager::NetworkManagerProxy;
use std::collections::HashMap;
use std::env;
use zbus::Connection;
use zbus::zvariant::Value;

#[tokio::main]
async fn main() {
    let connection = Connection::system()
        .await
        .expect("Could not get system connection");

    let nm = NetworkManagerProxy::new(&connection)
        .await
        .expect("Could not get NetworkManager");

    let wireless_interface_path: zbus::zvariant::OwnedObjectPath = nm
        .get_device_by_ip_iface("wlan0")
        .await
        .expect("Could not get wireless interface");

    tokio::join! {
        connect(&nm, wireless_interface_path)
    };
}

async fn connect(
    nm: &NetworkManagerProxy<'_>,
    wireless_interface_path: zbus::zvariant::OwnedObjectPath,
) {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: ./add_connection <connection_name> <ssid> <psk>");
        eprintln!("Example: ./add_connection MyConnectionName MySSID MyPassword");
        std::process::exit(1);
    }

    let connection_name = &args[1];
    let ssid = &args[2];
    let psk = &args[3];

    let mut connection_settings: HashMap<&str, HashMap<&str, Value<'_>>> = HashMap::new();

    let mut connection: HashMap<&str, Value<'_>> = HashMap::new();
    connection.insert("id", Value::new(connection_name));
    connection.insert("type", Value::new("802-11-wireless"));
    connection.insert("autoconnect", Value::new(true));
    connection_settings.insert("connection", connection);

    let mut wireless80211: HashMap<&str, Value<'_>> = HashMap::new();
    // SSID must be a Vec<u8>
    wireless80211.insert("ssid", Value::new(ssid.as_bytes()));
    wireless80211.insert("mode", Value::new("infrastructure"));
    connection_settings.insert("802-11-wireless", wireless80211);

    let mut wireless_security: HashMap<&str, Value<'_>> = HashMap::new();
    wireless_security.insert("key-mgmt", Value::new("wpa-psk"));
    wireless_security.insert("psk", Value::new(psk));
    connection_settings.insert("802-11-wireless-security", wireless_security);

    let mut ipv4: HashMap<&str, Value<'_>> = HashMap::new();
    ipv4.insert("method", Value::new("auto"));
    connection_settings.insert("ipv4", ipv4);

    let mut ipv6: HashMap<&str, Value<'_>> = HashMap::new();
    ipv6.insert("method", Value::new("auto"));
    connection_settings.insert("ipv6", ipv6);

    let specific_object = zbus::zvariant::OwnedObjectPath::try_from("/").unwrap();

    nm.add_and_activate_connection(
        connection_settings,
        &wireless_interface_path,
        &specific_object,
    )
    .await
    .expect("Could not add and activate connection");
}
