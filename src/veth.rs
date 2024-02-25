//! # D-Bus interface proxy for: `org.freedesktop.NetworkManager.Device.Veth`
//!
//! This code was generated by `zbus-xmlgen` `4.0.1` from D-Bus introspection data.
//! Source: `org.freedesktop.NetworkManager.Device.Veth.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the [Writing a client proxy] section of the zbus
//! documentation.
//!
//!
//! [Writing a client proxy]: https://dbus2.github.io/zbus/client.html
//! [D-Bus standard interfaces]: https://dbus.freedesktop.org/doc/dbus-specification.html#standard-interfaces,
use zbus::proxy;
#[proxy(
    interface = "org.freedesktop.NetworkManager.Device.Veth",
    assume_defaults = true
)]
trait Veth {
    /// Peer property
    #[zbus(property)]
    fn peer(&self) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
}
