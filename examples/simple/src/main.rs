use network_manager::{Connection, NetworkManagerProxy, DeviceProxy, WirelessProxy};

#[tokio::main]
async fn main() {
    let connection = Connection::system().await.expect("Could not get a connection.");

    let nm = NetworkManagerProxy::new(&connection)
        .await
        .expect("Could not get NetworkManager");

    match nm.get_device_by_ip_iface("wlan0").await {
        Ok(wireless_path) => {
            let wireless_proxy = WirelessProxy::new_from_path(wireless_path, &connection);
            println!("Found wireless proxy");
        },
        Err(_) => println!("Wireless device not found!"),
    };

    for device in nm.get_all_devices().await.expect("Could not find devices") {
        let device_proxy = DeviceProxy::new_from_path(device, &connection).await.expect("Error");

        let p = device_proxy.path().await;

        let interface = device_proxy.interface().await;
        let hw_address = device_proxy.hw_address().await;

        println!("Network device: {:?} ({:?})", interface, hw_address);

        match p {
            Ok(path) => println!("Device: {}", path),
            Err(_) => println!("Could not find path"),
        };
    }
}
