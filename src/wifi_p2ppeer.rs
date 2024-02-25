//! # D-Bus interface proxy for: `org.freedesktop.NetworkManager.WifiP2PPeer`
//!
//! This code was generated by `zbus-xmlgen` `4.0.1` from D-Bus introspection data.
//! Source: `org.freedesktop.NetworkManager.WifiP2PPeer.xml`.
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
    interface = "org.freedesktop.NetworkManager.WifiP2PPeer",
    assume_defaults = true
)]
trait WifiP2PPeer {
    /// Flags property
    #[zbus(property)]
    fn flags(&self) -> zbus::Result<u32>;

    /// HwAddress property
    #[zbus(property)]
    fn hw_address(&self) -> zbus::Result<String>;

    /// LastSeen property
    #[zbus(property)]
    fn last_seen(&self) -> zbus::Result<i32>;

    /// Manufacturer property
    #[zbus(property)]
    fn manufacturer(&self) -> zbus::Result<String>;

    /// Model property
    #[zbus(property)]
    fn model(&self) -> zbus::Result<String>;

    /// ModelNumber property
    #[zbus(property)]
    fn model_number(&self) -> zbus::Result<String>;

    /// Name property
    #[zbus(property)]
    fn name(&self) -> zbus::Result<String>;

    /// Serial property
    #[zbus(property)]
    fn serial(&self) -> zbus::Result<String>;

    /// Strength property
    #[zbus(property)]
    fn strength(&self) -> zbus::Result<u8>;

    /// WfdIEs property
    #[zbus(property, name = "WfdIEs")]
    fn wfd_ies(&self) -> zbus::Result<Vec<u8>>;
}
