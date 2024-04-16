//! # D-Bus interface proxy for: `org.freedesktop.NetworkManager.Device.Vlan`
//!
//! This code was generated by `zbus-xmlgen` `4.0.1` from D-Bus introspection data.
//! Source: `org.freedesktop.NetworkManager.Device.Vlan.xml`.
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

impl VlanProxy<'_> {
    pub async fn new_from_path(
        device_path: zbus::zvariant::OwnedObjectPath,
        connection: &Connection,
    ) -> Result<VlanProxy<'_>> {
        VlanProxy::builder(&connection)
            .path(device_path)
            .expect("Path not found")
            .build()
            .await
    }
}

#[proxy(
    default_path = "/org/freedesktop/NetworkManager/Device/Vlan",
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Vlan",
    assume_defaults = true
)]
trait Vlan {
    /// Carrier property
    #[zbus(property)]
    fn carrier(&self) -> zbus::Result<bool>;

    /// HwAddress property
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// Parent property
    #[zbus(property)]
    fn parent(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// VlanId property
    #[zbus(property)]
    fn vlan_id(&self) -> zbus::Result<u32>;
}
