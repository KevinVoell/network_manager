//! Device interface flags for NetworkManager devices
//!
//! This module contains the `DeviceInterfaceFlags` bitflags that represent the various flags
//! that can be set on a NetworkManager device interface.

use bitflags::bitflags;

bitflags! {
    /// NetworkManager device interface flags
    /// 
    /// These flags correspond to the `NMDeviceInterfaceFlags` flags in NetworkManager's D-Bus interface.
    /// The numeric values match those defined in the NetworkManager source code.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DeviceInterfaceFlags: u32 {
        /// No flags set
        const NONE = 0;
        /// The interface is up (corresponds to kernel IFF_UP)
        const UP = 0x1;
        /// The physical link is up (corresponds to kernel IFF_LOWER_UP)
        const LOWER_UP = 0x2;
        /// The interface has carrier (NetworkManager-specific flag)
        const CARRIER = 0x10000;
    }
}

impl From<u32> for DeviceInterfaceFlags {
    /// Convert a raw NetworkManager device interface flags value to typed bitflags
    fn from(value: u32) -> Self {
        DeviceInterfaceFlags::from_bits_truncate(value)
    }
}

impl From<DeviceInterfaceFlags> for u32 {
    /// Convert DeviceInterfaceFlags bitflags back to their raw NetworkManager value
    fn from(flags: DeviceInterfaceFlags) -> Self {
        flags.bits()
    }
}

impl std::fmt::Display for DeviceInterfaceFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return write!(f, "None");
        }
        
        let mut parts = Vec::new();
        
        if self.contains(DeviceInterfaceFlags::UP) {
            parts.push("Up");
        }
        if self.contains(DeviceInterfaceFlags::LOWER_UP) {
            parts.push("Lower Up");
        }
        if self.contains(DeviceInterfaceFlags::CARRIER) {
            parts.push("Carrier");
        }
        
        write!(f, "{}", parts.join(", "))
    }
}

impl DeviceInterfaceFlags {
    /// Returns true if the interface is up
    pub fn is_up(&self) -> bool {
        self.contains(DeviceInterfaceFlags::UP)
    }
    
    /// Returns true if the physical link is up
    pub fn is_lower_up(&self) -> bool {
        self.contains(DeviceInterfaceFlags::LOWER_UP)
    }
    
    /// Returns true if the interface has carrier
    pub fn has_carrier(&self) -> bool {
        self.contains(DeviceInterfaceFlags::CARRIER)
    }
    
    /// Returns true if the interface is fully operational (up, lower up, and has carrier)
    pub fn is_operational(&self) -> bool {
        self.contains(DeviceInterfaceFlags::UP | DeviceInterfaceFlags::LOWER_UP | DeviceInterfaceFlags::CARRIER)
    }
}