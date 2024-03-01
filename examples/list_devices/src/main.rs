use network_manager::{Connection, NetworkManagerProxy, DeviceProxy};

#[tokio::main]
async fn main() {
    let connection = Connection::system()
        .await
        .expect("Could not get a connection.");

    let nm = NetworkManagerProxy::new(&connection)
        .await
        .expect("Could not get NetworkManager");

    for device in nm.get_all_devices().await.expect("Could not find devices") {
        let device_proxy = DeviceProxy::new_from_path(device, &connection).await.expect("Error");

        let path = device_proxy.path().await;

        let interface = device_proxy.interface().await;
        let hw_address = device_proxy.hw_address().await;

        match path {
            Ok(path) => println!("Device Path: {}\n\t interface: {}\n\t hw_address: {}", 
                                    path, 
                                    interface.unwrap(), 
                                    hw_address.unwrap()),
            Err(_) => println!("Could not find path"),
        };
    }
}