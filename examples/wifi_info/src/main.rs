use network_manager::{AccessPointProxy, NetworkManagerProxy, WirelessProxy};
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

            let access_point_path = wireless_proxy.active_access_point().await;

            let access_point =
                AccessPointProxy::new_from_path(access_point_path.unwrap(), &connection)
                    .await
                    .expect("Unable to get the access point");

            let ssid = access_point.ssid().await;
            let frequency = access_point.frequency().await;
            let hw_address = access_point.hw_address().await;

            println!(
                "Access Point: {}\n\tFrequency: {:?}\n\tAddress: {}",
                String::from_utf8(ssid.unwrap()).unwrap(),
                frequency.unwrap(),
                hw_address.unwrap()
            );
        }
        Err(_) => println!("Wireless device not found!"),
    };
}
