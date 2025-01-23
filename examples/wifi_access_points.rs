//! # wifi_access_points Example
//!
//! Rust example that displays the all the access points that an interface can see.
//! Usage: ./wifi_access_points <ifname>
//!
//! Example: ./wifi_access_points wlan0
//!
//! DISCLAIMER:
//! The example code provided here is for illustrative purposes only. It is provided "AS IS",
//! without warranty of any kind, express or implied, including but not limited to the warranties of
//! merchantability, fitness for a particular purpose, and non-infringement. In no event shall the authors
//! or copyright holders be liable for any claim, damages, or other liability, whether in an action of
//! contract, tort, or otherwise, arising from, out of, or in connection with the example code or
//! the use or other dealings in the example code.

use rusty_network_manager::{AccessPointProxy, NetworkManagerProxy, WirelessProxy};
use std::collections::HashMap;
use tokio_stream::StreamExt;
use zbus::Connection;

#[tokio::main]
async fn main() {
    let connection = Connection::system()
        .await
        .expect("Could not get system connection");

    let nm = NetworkManagerProxy::new(&connection)
        .await
        .expect("Could not get NetworkManager");

    let wireless_interface_path = nm
        .get_device_by_ip_iface("wlan0")
        .await
        .expect("Could not get wireless interface");

    let wireless_proxy = WirelessProxy::new_from_path(wireless_interface_path, &connection)
        .await
        .expect("Could not get wireless device");

    tokio::join! {
        monitor_add_access_point(&wireless_proxy, &connection),
        monitor_remove_access_point(&wireless_proxy, &connection),
        request_scan(&wireless_proxy)
    };
}

async fn request_scan(wireless_proxy: &WirelessProxy<'_>) {
    let _last_scan = wireless_proxy.last_scan().await;

    tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    let options = HashMap::new();
    wireless_proxy
        .request_scan(options)
        .await
        .expect("Could not request scan, try running as sudo.");
    println!("Scanning for access points");
}

async fn monitor_add_access_point(wireless_proxy: &WirelessProxy<'_>, connection: &Connection) {
    println!("Monitoring for new APs");
    let mut access_points_changed = wireless_proxy
        .receive_access_point_added()
        .await
        .expect("We got here!");
    while let Some(v) = access_points_changed.next().await {
        let args = v.args().unwrap();

        let owned_object_path = zbus::zvariant::OwnedObjectPath::from(args.access_point);

        let access_point = AccessPointProxy::new_from_path(owned_object_path, connection)
            .await
            .expect("Could not get access point");

        let ssid = String::from_utf8(access_point.ssid().await.unwrap());
        println!("+ {} {:?}", ssid.unwrap(), v.message());
    }
}

async fn monitor_remove_access_point(wireless_proxy: &WirelessProxy<'_>, connection: &Connection) {
    println!("Monitoring for removed APs");
    let mut access_points_changed = wireless_proxy
        .receive_access_point_removed()
        .await
        .expect("We got here!");
    while let Some(v) = access_points_changed.next().await {
        let args = v.args().unwrap();

        let owned_object_path = zbus::zvariant::OwnedObjectPath::from(args.access_point);

        let access_point = AccessPointProxy::new_from_path(owned_object_path, connection)
            .await
            .expect("Could not get access point");

        let ssid = String::from_utf8(access_point.ssid().await.unwrap());
        println!("- {} {}", ssid.unwrap(), v.message());
    }
}
