//! # D-Bus interface proxy for: `org.freedesktop.NetworkManager.Device.Statistics`
//!
//! This code was generated by `zbus-xmlgen` `4.0.1` from D-Bus introspection data.
//! Source: `org.freedesktop.NetworkManager.Device.Statistics.xml`.
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
    interface = "org.freedesktop.NetworkManager.Device.Statistics",
    assume_defaults = true
)]
trait Statistics {
    /// RefreshRateMs property
    #[zbus(property)]
    fn refresh_rate_ms(&self) -> zbus::Result<u32>;
    #[zbus(property)]
    fn set_refresh_rate_ms(&self, value: u32) -> zbus::Result<()>;

    /// RxBytes property
    #[zbus(property)]
    fn rx_bytes(&self) -> zbus::Result<u64>;

    /// TxBytes property
    #[zbus(property)]
    fn tx_bytes(&self) -> zbus::Result<u64>;
}
