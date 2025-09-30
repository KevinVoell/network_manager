//! Device capabilities bitflags for NetworkManager devices
//!
//! This module contains the `DeviceCapabilities` bitflags that represent the various capabilities
//! that a NetworkManager device may have.

use bitflags::bitflags;

bitflags! {
    /// NetworkManager device capabilities
    /// 
    /// These capabilities correspond to the `NMDeviceCapabilities` flags in NetworkManager's D-Bus interface.
    /// The numeric values match those defined in the NetworkManager source code.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct DeviceCapabilities: u32 {
        /// Device has no special capabilities
        const NONE = 0x00000000;
        /// NetworkManager supports this device
        const NM_SUPPORTED = 0x00000001;
        /// This device can indicate carrier status
        const CARRIER_DETECT = 0x00000002;
        /// This device is a software device
        const IS_SOFTWARE = 0x00000004;
        /// This device supports single-root I/O virtualization
        const SRIOV = 0x00000008;
    }
}

impl From<u32> for DeviceCapabilities {
    /// Convert a raw NetworkManager device capabilities value to typed bitflags
    fn from(value: u32) -> Self {
        DeviceCapabilities::from_bits_truncate(value)
    }
}

impl From<DeviceCapabilities> for u32 {
    /// Convert DeviceCapabilities bitflags back to their raw NetworkManager value
    fn from(capabilities: DeviceCapabilities) -> Self {
        capabilities.bits()
    }
}

impl std::fmt::Display for DeviceCapabilities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return write!(f, "None");
        }
        
        let mut parts = Vec::new();
        
        if self.contains(DeviceCapabilities::NM_SUPPORTED) {
            parts.push("NetworkManager Supported");
        }
        if self.contains(DeviceCapabilities::CARRIER_DETECT) {
            parts.push("Carrier Detection");
        }
        if self.contains(DeviceCapabilities::IS_SOFTWARE) {
            parts.push("Software Device");
        }
        if self.contains(DeviceCapabilities::SRIOV) {
            parts.push("SR-IOV");
        }
        
        write!(f, "{}", parts.join(", "))
    }
}