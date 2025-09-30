//! Device state enumeration for NetworkManager devices
//!
//! This module contains the `DeviceState` enum that represents the various states
//! a NetworkManager device can be in, along with conversion utilities.

/// NetworkManager device states
/// 
/// These states correspond to the `NMDeviceState` enum in NetworkManager's D-Bus interface.
/// The numeric values match those defined in the NetworkManager source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    /// The device's state is unknown
    Unknown,
    /// The device is recognized, but not managed by NetworkManager
    Unmanaged,
    /// The device is managed by NetworkManager, but is not available for use
    Unavailable,
    /// The device can be activated, but is currently idle and not connected to a network
    Disconnected,
    /// The device is preparing the connection to the network
    Prepare,
    /// The device is connecting to the requested network
    Config,
    /// The device requires more information to continue connecting to the requested network
    NeedAuth,
    /// The device is requesting IPv4 and/or IPv6 addresses and routing information from the network
    IpConfig,
    /// The device is checking whether further action is required for the requested network connection
    IpCheck,
    /// The device is waiting for a secondary connection (like a VPN) which must be activated before the device can be activated
    Secondaries,
    /// The device has a network connection, either local or global
    Activated,
    /// A disconnection from the current network connection was requested, and the device is cleaning up resources used for that connection
    Deactivating,
    /// The device failed to connect to the requested network and is cleaning up the connection request
    Failed,
}

impl From<u32> for DeviceState {
    /// Convert a raw NetworkManager device state value to a typed enum
    fn from(value: u32) -> Self {
        match value {
            0 => DeviceState::Unknown,
            10 => DeviceState::Unmanaged,
            20 => DeviceState::Unavailable,
            30 => DeviceState::Disconnected,
            40 => DeviceState::Prepare,
            50 => DeviceState::Config,
            60 => DeviceState::NeedAuth,
            70 => DeviceState::IpConfig,
            80 => DeviceState::IpCheck,
            90 => DeviceState::Secondaries,
            100 => DeviceState::Activated,
            110 => DeviceState::Deactivating,
            120 => DeviceState::Failed,
            _ => DeviceState::Unknown,
        }
    }
}

impl From<DeviceState> for u32 {
    /// Convert a DeviceState enum value back to its raw NetworkManager value
    fn from(state: DeviceState) -> Self {
        match state {
            DeviceState::Unknown => 0,
            DeviceState::Unmanaged => 10,
            DeviceState::Unavailable => 20,
            DeviceState::Disconnected => 30,
            DeviceState::Prepare => 40,
            DeviceState::Config => 50,
            DeviceState::NeedAuth => 60,
            DeviceState::IpConfig => 70,
            DeviceState::IpCheck => 80,
            DeviceState::Secondaries => 90,
            DeviceState::Activated => 100,
            DeviceState::Deactivating => 110,
            DeviceState::Failed => 120,
        }
    }
}

impl std::fmt::Display for DeviceState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_str = match self {
            DeviceState::Unknown => "Unknown",
            DeviceState::Unmanaged => "Unmanaged",
            DeviceState::Unavailable => "Unavailable",
            DeviceState::Disconnected => "Disconnected",
            DeviceState::Prepare => "Prepare",
            DeviceState::Config => "Config",
            DeviceState::NeedAuth => "Need Auth",
            DeviceState::IpConfig => "IP Config",
            DeviceState::IpCheck => "IP Check",
            DeviceState::Secondaries => "Secondaries",
            DeviceState::Activated => "Activated",
            DeviceState::Deactivating => "Deactivating",
            DeviceState::Failed => "Failed",
        };
        write!(f, "{}", state_str)
    }
}