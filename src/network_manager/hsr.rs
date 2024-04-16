//! # D-Bus interface proxy for: `org.freedesktop.NetworkManager.Device.Hsr`
//!
//! This code was generated by `zbus-xmlgen` `4.0.1` from D-Bus introspection data.
//! Source: `org.freedesktop.NetworkManager.Device.Hsr.xml`.
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

impl HsrProxy<'_> {
    pub async fn new_from_path(
        device_path: zbus::zvariant::OwnedObjectPath,
        connection: &Connection,
    ) -> Result<HsrProxy<'_>> {
        HsrProxy::builder(&connection)
            .path(device_path)
            .expect("Path not found")
            .build()
            .await
    }
}

#[proxy(
    default_path = "/org/freedesktop/NetworkManager/Device/Hsr",
    default_service = "org.freedesktop.NetworkManager",
    interface = "org.freedesktop.NetworkManager.Device.Hsr",
    assume_defaults = true
)]
trait Hsr {
    /// MulticastSpec property
    #[zbus(property)]
    fn multicast_spec(&self) -> zbus::Result<u8>;

    /// Port1 property
    #[zbus(property)]
    fn port1(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Port2 property
    #[zbus(property)]
    fn port2(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;

    /// Prp property
    #[zbus(property)]
    fn prp(&self) -> zbus::Result<bool>;

    /// SupervisionAddress property
    #[zbus(property)]
    fn supervision_address(&self) -> zbus::Result<String>;
}
