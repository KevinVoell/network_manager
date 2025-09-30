//! Device state reason enumeration for NetworkManager devices
//!
//! This module contains the `DeviceStateReason` enum that represents the reasons
//! why a device changed its state.

/// NetworkManager device state change reasons
/// 
/// These reasons correspond to the `NMDeviceStateReason` enum in NetworkManager's D-Bus interface.
/// The numeric values match those defined in the NetworkManager source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceStateReason {
    /// No reason given
    None,
    /// Unknown error
    Unknown,
    /// Device is now managed
    NowManaged,
    /// Device is now unmanaged
    NowUnmanaged,
    /// The device could not be readied for configuration
    ConfigFailed,
    /// IP configuration could not be reserved (no available address, timeout, etc)
    IpConfigUnavailable,
    /// The IP config is no longer valid
    IpConfigExpired,
    /// Secrets were required, but not provided
    NoSecrets,
    /// 802.1x supplicant disconnected
    SupplicantDisconnect,
    /// 802.1x supplicant configuration failed
    SupplicantConfigFailed,
    /// 802.1x supplicant failed
    SupplicantFailed,
    /// 802.1x supplicant took too long to authenticate
    SupplicantTimeout,
    /// PPP service failed to start
    PppStartFailed,
    /// PPP service disconnected
    PppDisconnect,
    /// PPP failed
    PppFailed,
    /// DHCP client failed to start
    DhcpStartFailed,
    /// DHCP client error
    DhcpError,
    /// DHCP client failed
    DhcpFailed,
    /// Shared connection service failed to start
    SharedStartFailed,
    /// Shared connection service failed
    SharedFailed,
    /// AutoIP service failed to start
    AutoipStartFailed,
    /// AutoIP service error
    AutoipError,
    /// AutoIP service failed
    AutoipFailed,
    /// The line is busy
    ModemBusy,
    /// No dial tone
    ModemNoDialTone,
    /// No carrier could be established
    ModemNoCarrier,
    /// The dialing request timed out
    ModemDialTimeout,
    /// The dialing attempt failed
    ModemDialFailed,
    /// Modem initialization failed
    ModemInitFailed,
    /// Failed to select the specified APN
    GsmApnFailed,
    /// Not searching for networks
    GsmRegistrationNotSearching,
    /// Network registration denied
    GsmRegistrationDenied,
    /// Network registration timed out
    GsmRegistrationTimeout,
    /// Failed to register with the requested network
    GsmRegistrationFailed,
    /// PIN check failed
    GsmPinCheckFailed,
    /// Necessary firmware for the device may be missing
    FirmwareMissing,
    /// The device was removed
    Removed,
    /// NetworkManager went to sleep
    Sleeping,
    /// The device's active connection disappeared
    ConnectionRemoved,
    /// Device disconnected by user or client
    UserRequested,
    /// Carrier/link changed
    Carrier,
    /// The device's existing connection was assumed
    ConnectionAssumed,
    /// The supplicant is now available
    SupplicantAvailable,
    /// The modem could not be found
    ModemNotFound,
    /// The Bluetooth connection failed or timed out
    BtFailed,
    /// GSM Modem's SIM Card not inserted
    GsmSimNotInserted,
    /// GSM Modem's SIM Pin required
    GsmSimPinRequired,
    /// GSM Modem's SIM Puk required
    GsmSimPukRequired,
    /// GSM Modem's SIM wrong
    GsmSimWrong,
    /// InfiniBand device does not support connected mode
    InfinibandMode,
    /// A dependency of the connection failed
    DependencyFailed,
    /// Problem with the RFC 2684 Ethernet over ADSL bridge
    Br2684Failed,
    /// ModemManager not running
    ModemManagerUnavailable,
    /// The Wi-Fi network could not be found
    SsidNotFound,
    /// A secondary connection of the base connection failed
    SecondaryConnectionFailed,
    /// DCB or FCoE setup failed
    DcbFcoeFailed,
    /// teamd control failed
    TeamdControlFailed,
    /// Modem failed or no longer available
    ModemFailed,
    /// Modem now ready and available
    ModemAvailable,
    /// SIM PIN was incorrect
    SimPinIncorrect,
    /// New connection activation was enqueued
    NewActivation,
    /// The device's parent changed
    ParentChanged,
    /// The device parent's management changed
    ParentManagedChanged,
    /// Problem communicating with Open vSwitch database
    OvsdbFailed,
    /// A duplicate IP address was detected
    IpAddressDuplicate,
    /// The selected IP method is not supported
    IpMethodUnsupported,
    /// Configuration of SR-IOV parameters failed
    SriovConfigurationFailed,
    /// The Wi-Fi P2P peer could not be found
    PeerNotFound,
}

impl From<u32> for DeviceStateReason {
    /// Convert a raw NetworkManager device state reason value to a typed enum
    fn from(value: u32) -> Self {
        match value {
            0 => DeviceStateReason::None,
            1 => DeviceStateReason::Unknown,
            2 => DeviceStateReason::NowManaged,
            3 => DeviceStateReason::NowUnmanaged,
            4 => DeviceStateReason::ConfigFailed,
            5 => DeviceStateReason::IpConfigUnavailable,
            6 => DeviceStateReason::IpConfigExpired,
            7 => DeviceStateReason::NoSecrets,
            8 => DeviceStateReason::SupplicantDisconnect,
            9 => DeviceStateReason::SupplicantConfigFailed,
            10 => DeviceStateReason::SupplicantFailed,
            11 => DeviceStateReason::SupplicantTimeout,
            12 => DeviceStateReason::PppStartFailed,
            13 => DeviceStateReason::PppDisconnect,
            14 => DeviceStateReason::PppFailed,
            15 => DeviceStateReason::DhcpStartFailed,
            16 => DeviceStateReason::DhcpError,
            17 => DeviceStateReason::DhcpFailed,
            18 => DeviceStateReason::SharedStartFailed,
            19 => DeviceStateReason::SharedFailed,
            20 => DeviceStateReason::AutoipStartFailed,
            21 => DeviceStateReason::AutoipError,
            22 => DeviceStateReason::AutoipFailed,
            23 => DeviceStateReason::ModemBusy,
            24 => DeviceStateReason::ModemNoDialTone,
            25 => DeviceStateReason::ModemNoCarrier,
            26 => DeviceStateReason::ModemDialTimeout,
            27 => DeviceStateReason::ModemDialFailed,
            28 => DeviceStateReason::ModemInitFailed,
            29 => DeviceStateReason::GsmApnFailed,
            30 => DeviceStateReason::GsmRegistrationNotSearching,
            31 => DeviceStateReason::GsmRegistrationDenied,
            32 => DeviceStateReason::GsmRegistrationTimeout,
            33 => DeviceStateReason::GsmRegistrationFailed,
            34 => DeviceStateReason::GsmPinCheckFailed,
            35 => DeviceStateReason::FirmwareMissing,
            36 => DeviceStateReason::Removed,
            37 => DeviceStateReason::Sleeping,
            38 => DeviceStateReason::ConnectionRemoved,
            39 => DeviceStateReason::UserRequested,
            40 => DeviceStateReason::Carrier,
            41 => DeviceStateReason::ConnectionAssumed,
            42 => DeviceStateReason::SupplicantAvailable,
            43 => DeviceStateReason::ModemNotFound,
            44 => DeviceStateReason::BtFailed,
            45 => DeviceStateReason::GsmSimNotInserted,
            46 => DeviceStateReason::GsmSimPinRequired,
            47 => DeviceStateReason::GsmSimPukRequired,
            48 => DeviceStateReason::GsmSimWrong,
            49 => DeviceStateReason::InfinibandMode,
            50 => DeviceStateReason::DependencyFailed,
            51 => DeviceStateReason::Br2684Failed,
            52 => DeviceStateReason::ModemManagerUnavailable,
            53 => DeviceStateReason::SsidNotFound,
            54 => DeviceStateReason::SecondaryConnectionFailed,
            55 => DeviceStateReason::DcbFcoeFailed,
            56 => DeviceStateReason::TeamdControlFailed,
            57 => DeviceStateReason::ModemFailed,
            58 => DeviceStateReason::ModemAvailable,
            59 => DeviceStateReason::SimPinIncorrect,
            60 => DeviceStateReason::NewActivation,
            61 => DeviceStateReason::ParentChanged,
            62 => DeviceStateReason::ParentManagedChanged,
            63 => DeviceStateReason::OvsdbFailed,
            64 => DeviceStateReason::IpAddressDuplicate,
            65 => DeviceStateReason::IpMethodUnsupported,
            66 => DeviceStateReason::SriovConfigurationFailed,
            67 => DeviceStateReason::PeerNotFound,
            _ => DeviceStateReason::Unknown,
        }
    }
}

impl std::fmt::Display for DeviceStateReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reason_str = match self {
            DeviceStateReason::None => "No reason",
            DeviceStateReason::Unknown => "Unknown error",
            DeviceStateReason::NowManaged => "Device is now managed",
            DeviceStateReason::NowUnmanaged => "Device is now unmanaged",
            DeviceStateReason::ConfigFailed => "Configuration failed",
            DeviceStateReason::IpConfigUnavailable => "IP configuration unavailable",
            DeviceStateReason::IpConfigExpired => "IP configuration expired",
            DeviceStateReason::NoSecrets => "Secrets required but not provided",
            DeviceStateReason::SupplicantDisconnect => "802.1x supplicant disconnected",
            DeviceStateReason::SupplicantConfigFailed => "802.1x supplicant configuration failed",
            DeviceStateReason::SupplicantFailed => "802.1x supplicant failed",
            DeviceStateReason::SupplicantTimeout => "802.1x supplicant timeout",
            DeviceStateReason::PppStartFailed => "PPP start failed",
            DeviceStateReason::PppDisconnect => "PPP disconnected",
            DeviceStateReason::PppFailed => "PPP failed",
            DeviceStateReason::DhcpStartFailed => "DHCP start failed",
            DeviceStateReason::DhcpError => "DHCP error",
            DeviceStateReason::DhcpFailed => "DHCP failed",
            DeviceStateReason::SharedStartFailed => "Shared connection start failed",
            DeviceStateReason::SharedFailed => "Shared connection failed",
            DeviceStateReason::AutoipStartFailed => "AutoIP start failed",
            DeviceStateReason::AutoipError => "AutoIP error",
            DeviceStateReason::AutoipFailed => "AutoIP failed",
            DeviceStateReason::ModemBusy => "Modem busy",
            DeviceStateReason::ModemNoDialTone => "No dial tone",
            DeviceStateReason::ModemNoCarrier => "No carrier",
            DeviceStateReason::ModemDialTimeout => "Dial timeout",
            DeviceStateReason::ModemDialFailed => "Dial failed",
            DeviceStateReason::ModemInitFailed => "Modem initialization failed",
            DeviceStateReason::GsmApnFailed => "GSM APN failed",
            DeviceStateReason::GsmRegistrationNotSearching => "GSM not searching for networks",
            DeviceStateReason::GsmRegistrationDenied => "GSM registration denied",
            DeviceStateReason::GsmRegistrationTimeout => "GSM registration timeout",
            DeviceStateReason::GsmRegistrationFailed => "GSM registration failed",
            DeviceStateReason::GsmPinCheckFailed => "GSM PIN check failed",
            DeviceStateReason::FirmwareMissing => "Firmware missing",
            DeviceStateReason::Removed => "Device removed",
            DeviceStateReason::Sleeping => "NetworkManager sleeping",
            DeviceStateReason::ConnectionRemoved => "Connection removed",
            DeviceStateReason::UserRequested => "User requested disconnection",
            DeviceStateReason::Carrier => "Carrier changed",
            DeviceStateReason::ConnectionAssumed => "Connection assumed",
            DeviceStateReason::SupplicantAvailable => "Supplicant available",
            DeviceStateReason::ModemNotFound => "Modem not found",
            DeviceStateReason::BtFailed => "Bluetooth failed",
            DeviceStateReason::GsmSimNotInserted => "SIM card not inserted",
            DeviceStateReason::GsmSimPinRequired => "SIM PIN required",
            DeviceStateReason::GsmSimPukRequired => "SIM PUK required",
            DeviceStateReason::GsmSimWrong => "Wrong SIM card",
            DeviceStateReason::InfinibandMode => "InfiniBand mode not supported",
            DeviceStateReason::DependencyFailed => "Dependency failed",
            DeviceStateReason::Br2684Failed => "RFC 2684 bridge failed",
            DeviceStateReason::ModemManagerUnavailable => "ModemManager unavailable",
            DeviceStateReason::SsidNotFound => "Wi-Fi network not found",
            DeviceStateReason::SecondaryConnectionFailed => "Secondary connection failed",
            DeviceStateReason::DcbFcoeFailed => "DCB or FCoE failed",
            DeviceStateReason::TeamdControlFailed => "Teamd control failed",
            DeviceStateReason::ModemFailed => "Modem failed",
            DeviceStateReason::ModemAvailable => "Modem available",
            DeviceStateReason::SimPinIncorrect => "SIM PIN incorrect",
            DeviceStateReason::NewActivation => "New activation enqueued",
            DeviceStateReason::ParentChanged => "Parent device changed",
            DeviceStateReason::ParentManagedChanged => "Parent management changed",
            DeviceStateReason::OvsdbFailed => "Open vSwitch database failed",
            DeviceStateReason::IpAddressDuplicate => "Duplicate IP address",
            DeviceStateReason::IpMethodUnsupported => "IP method unsupported",
            DeviceStateReason::SriovConfigurationFailed => "SR-IOV configuration failed",
            DeviceStateReason::PeerNotFound => "Wi-Fi P2P peer not found",
        };
        write!(f, "{}", reason_str)
    }
}