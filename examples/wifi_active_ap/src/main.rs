//! # wifi_active_ap Example
//! 
//! Rust example that displays the current access point.
//! Usage: ./wifi_active_ap <ifname>
//!
//! Example: ./wifi_active_ap wlan0
//!
//! This example is based off an official example written in Python available here:
//! https://gitlab.freedesktop.org/NetworkManager/NetworkManager/-/blob/main/examples/python/dbus/wifi-active-ap.py
//! 
//! DISCLAIMER:
//! The example code provided here is for illustrative purposes only. It is provided "AS IS", 
//! without warranty of any kind, express or implied, including but not limited to the warranties of 
//! merchantability, fitness for a particular purpose, and non-infringement. In no event shall the authors 
//! or copyright holders be liable for any claim, damages, or other liability, whether in an action of 
//! contract, tort, or otherwise, arising from, out of, or in connection with the example code or 
//! the use or other dealings in the example code.

use std::env;
use network_manager::{Connection, NetworkManagerProxy, WirelessProxy, AccessPointProxy};

#[tokio::main]
async fn main() {
    let connection = Connection::system().await.expect("Could not get a connection.");

    let nm = NetworkManagerProxy::new(&connection)
        .await
        .expect("Could not get NetworkManager");

    //let settings_path = zbus::zvariant::OwnedObjectPath::try_from("org.freedesktop.NetworkManager.Settings".to_string());
    //let settings_proxy = SettingsProxy::new_from_path(settings_path.unwrap(), &connection);

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <ifname>", &args[0]);
        return
    } 

    let iface = &args[1];

    let dev_path = nm.get_device_by_ip_iface(iface).await;

    let wireless_proxy = WirelessProxy::new_from_path(dev_path.unwrap(), &connection)
        .await
        .expect("Could not get wireless device");
    
    let active_access_point_path = wireless_proxy.active_access_point()
        .await
        .expect("Could not get active access point path");

    if active_access_point_path.as_ref() == "/" {
        println!("{} is not currently associated", iface);
        return;
    }

    let active_proxy = AccessPointProxy::new_from_path(active_access_point_path, &connection)
        .await
        .expect("Could not get active access point");

    let ssid = String::from_utf8(active_proxy.ssid().await.expect("Could not get SSD")).unwrap(); 
    let hw_address = active_proxy.hw_address().await.expect("Could not get hw address");
    
    println!("{iface} is associated to '{ssid}' ({hw_address})");
}
