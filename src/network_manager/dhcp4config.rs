//! # D-Bus interface proxy for: `org.freedesktop.NetworkManager.DHCP4Config`
//!
//! This code was generated by `zbus-xmlgen` `4.0.1` from D-Bus introspection data.
//! Source: `org.freedesktop.NetworkManager.DHCP4Config.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,
use zbus::{proxy, Connection, Result};

impl DHCP4ConfigProxy<'_> {
    pub async fn new_from_path(
        device_path: zbus::zvariant::OwnedObjectPath,
        connection: &Connection,
    ) -> Result<DHCP4ConfigProxy<'_>> {
        DHCP4ConfigProxy::builder(&connection)
            .path(device_path)
            .expect("Path not found")
            .build()
            .await
    }
}

#[proxy(
    default_path = "/org/freedesktop/NetworkManager/DHCP4Config",
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.DHCP4Config",
    assume_defaults = true
)]
trait DHCP4Config {
    /// Options property
    #[zbus(property)]
    fn options(
        &self,
    ) -> zbus::Result<std::collections::HashMap<String, zbus::zvariant::OwnedValue>>;
}
