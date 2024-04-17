use network_manager::{DeviceProxy, NetworkManagerProxy};
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

        let p = device_proxy.path().await;

        let interface = device_proxy.interface().await;
        let hw_address = device_proxy.hw_address().await;

        println!(
            "Network device: {}\n\tInterface: {} ({})",
            p.unwrap(),
            interface.unwrap(),
            hw_address.unwrap()
        );
    }
}
