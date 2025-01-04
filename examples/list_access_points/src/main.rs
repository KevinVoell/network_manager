use std::collections::HashMap;
use tokio_stream::StreamExt;
use rusty_network_manager::{NetworkManagerProxy, WirelessProxy, AccessPointProxy};
use zbus::Connection;

#[tokio::main]
async fn main() {
    let connection = Connection::system()
        .await
        .expect("Could not get a connection.");

    let nm = NetworkManagerProxy::new(&connection)
        .await
        .expect("Could not get NetworkManager");

    match nm.get_device_by_ip_iface("wlan0").await {
        Ok(wireless_path) => {
            let wireless_proxy = WirelessProxy::new_from_path(wireless_path, &connection)
                .await
                .expect("Unable to get wireless proxy");

            let options = HashMap::new();
            wireless_proxy.request_scan(options).await.expect("Could not request scan, try running as sudo.");

            let mut access_points_changed = wireless_proxy.receive_access_point_added().await.expect("We got here!");
            while let Some(v) = access_points_changed.next().await {
                println!("{}", v.message());
            }

            // Get notified when scan is complete
            let mut scan_changed = wireless_proxy.receive_last_scan_changed().await;
            
            let mut count = 0;
            while let Some(v) = scan_changed.next().await {
                println!("{}", v.name());
                println!("scan changed: {:?}", v.get().await);
                count += 1;
                if count >= 2 {
                  //  break;
                }
            }
            
            let access_points = wireless_proxy.get_all_access_points().await;
            //println!("AccessPoints: {:?}", access_points);
            for access_point_path in access_points.unwrap() {
                let access_point = AccessPointProxy::new_from_path(access_point_path, &connection)
                    .await
                    .expect("Unable to get the access point");

                let ssid = access_point.ssid().await;
                let frequency = access_point.frequency().await;
                let hw_address = access_point.hw_address().await;

                let ssid_string = String::from_utf8(ssid.unwrap()).unwrap();
                if ssid_string == "Bender" {
                    continue;
                }

                println!("Access Point: {}\n\tFrequency: {:?}\n\tAddress: {}", ssid_string, frequency.unwrap(), hw_address.unwrap());
            }
        },
        Err(_) => println!("No access points found"),
    };
}