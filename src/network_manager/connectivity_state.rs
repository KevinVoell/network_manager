//! Connectivity state enumeration for NetworkManager devices
//!
//! This module contains the `ConnectivityState` enum that represents the level of
//! Internet connectivity available through a device.

/// NetworkManager connectivity states
/// 
/// These states correspond to the `NMConnectivityState` enum in NetworkManager's D-Bus interface.
/// The numeric values match those defined in the NetworkManager source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConnectivityState {
    /// Network connectivity is unknown
    Unknown,
    /// The host is not connected to any network
    None,
    /// The Internet connection is hijacked by a captive portal gateway
    Portal,
    /// The host is connected to a network, does not appear to be able to reach the full Internet, but a captive portal has not been detected
    Limited,
    /// The host is connected to a network, and appears to be able to reach the full Internet
    Full,
}

impl From<u32> for ConnectivityState {
    /// Convert a raw NetworkManager connectivity state value to a typed enum
    fn from(value: u32) -> Self {
        match value {
            0 => ConnectivityState::Unknown,
            1 => ConnectivityState::None,
            2 => ConnectivityState::Portal,
            3 => ConnectivityState::Limited,
            4 => ConnectivityState::Full,
            _ => ConnectivityState::Unknown,
        }
    }
}

impl From<ConnectivityState> for u32 {
    /// Convert a ConnectivityState enum value back to its raw NetworkManager value
    fn from(state: ConnectivityState) -> Self {
        match state {
            ConnectivityState::Unknown => 0,
            ConnectivityState::None => 1,
            ConnectivityState::Portal => 2,
            ConnectivityState::Limited => 3,
            ConnectivityState::Full => 4,
        }
    }
}

impl std::fmt::Display for ConnectivityState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_str = match self {
            ConnectivityState::Unknown => "Unknown",
            ConnectivityState::None => "No Connectivity",
            ConnectivityState::Portal => "Captive Portal",
            ConnectivityState::Limited => "Limited Connectivity",
            ConnectivityState::Full => "Full Connectivity",
        };
        write!(f, "{}", state_str)
    }
}

impl ConnectivityState {
    /// Returns true if there is any level of connectivity (Limited or Full)
    pub fn has_connectivity(&self) -> bool {
        matches!(self, ConnectivityState::Limited | ConnectivityState::Full)
    }
    
    /// Returns true if there is full Internet connectivity
    pub fn has_full_connectivity(&self) -> bool {
        matches!(self, ConnectivityState::Full)
    }
    
    /// Returns true if a captive portal is detected
    pub fn has_captive_portal(&self) -> bool {
        matches!(self, ConnectivityState::Portal)
    }
    
    /// Returns true if there is no connectivity at all
    pub fn has_no_connectivity(&self) -> bool {
        matches!(self, ConnectivityState::None)
    }
}