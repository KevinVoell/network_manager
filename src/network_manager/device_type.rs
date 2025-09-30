//! Device type enumeration for NetworkManager devices
//!
//! This module contains the `DeviceType` enum that represents the various types
//! of network devices that NetworkManager can manage.

/// NetworkManager device types
/// 
/// These types correspond to the `NMDeviceType` enum in NetworkManager's D-Bus interface.
/// The numeric values match those defined in the NetworkManager source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    /// Unknown device type
    Unknown,
    /// Generic support for unrecognized device types  
    Generic,
    /// A wired ethernet device
    Ethernet,
    /// An 802.11 Wi-Fi device
    Wifi,
    /// A Bluetooth device supporting PAN or DUN access protocols
    Bluetooth,
    /// An OLPC XO mesh networking device
    OlpcMesh,
    /// An 802.16e Mobile WiMAX broadband device
    Wimax,
    /// A modem supporting analog telephone, CDMA/EVDO, GSM/UMTS, or LTE network access protocols
    Modem,
    /// An IP-over-InfiniBand device
    Infiniband,
    /// A bond master interface
    Bond,
    /// An 802.1Q VLAN interface
    Vlan,
    /// ADSL modem
    Adsl,
    /// A bridge master interface
    Bridge,
    /// A team master interface
    Team,
    /// A TUN or TAP interface
    Tun,
    /// An IP tunnel interface
    IpTunnel,
    /// A MACVLAN interface
    Macvlan,
    /// A VXLAN interface
    Vxlan,
    /// A VETH interface
    Veth,
    /// A MACsec interface
    Macsec,
    /// A dummy interface
    Dummy,
    /// A PPP interface
    Ppp,
    /// An Open vSwitch interface
    OvsInterface,
    /// An Open vSwitch port
    OvsPort,
    /// An Open vSwitch bridge
    OvsBridge,
    /// A IEEE 802.15.4 (WPAN) MAC Layer Device
    Wpan,
    /// 6LoWPAN interface
    SixLowpan,
    /// A WireGuard interface
    Wireguard,
    /// An 802.11 Wi-Fi P2P device
    WifiP2p,
    /// A VRF (Virtual Routing and Forwarding) interface
    Vrf,
}

impl From<u32> for DeviceType {
    /// Convert a raw NetworkManager device type value to a typed enum
    fn from(value: u32) -> Self {
        match value {
            0 => DeviceType::Unknown,
            1 => DeviceType::Ethernet,
            2 => DeviceType::Wifi,
            5 => DeviceType::Bluetooth,
            6 => DeviceType::OlpcMesh,
            7 => DeviceType::Wimax,
            8 => DeviceType::Modem,
            9 => DeviceType::Infiniband,
            10 => DeviceType::Bond,
            11 => DeviceType::Vlan,
            12 => DeviceType::Adsl,
            13 => DeviceType::Bridge,
            14 => DeviceType::Generic,
            15 => DeviceType::Team,
            16 => DeviceType::Tun,
            17 => DeviceType::IpTunnel,
            18 => DeviceType::Macvlan,
            19 => DeviceType::Vxlan,
            20 => DeviceType::Veth,
            21 => DeviceType::Macsec,
            22 => DeviceType::Dummy,
            23 => DeviceType::Ppp,
            24 => DeviceType::OvsInterface,
            25 => DeviceType::OvsPort,
            26 => DeviceType::OvsBridge,
            27 => DeviceType::Wpan,
            28 => DeviceType::SixLowpan,
            29 => DeviceType::Wireguard,
            30 => DeviceType::WifiP2p,
            31 => DeviceType::Vrf,
            _ => DeviceType::Unknown,
        }
    }
}

impl From<DeviceType> for u32 {
    /// Convert a DeviceType enum value back to its raw NetworkManager value
    fn from(device_type: DeviceType) -> Self {
        match device_type {
            DeviceType::Unknown => 0,
            DeviceType::Ethernet => 1,
            DeviceType::Wifi => 2,
            DeviceType::Bluetooth => 5,
            DeviceType::OlpcMesh => 6,
            DeviceType::Wimax => 7,
            DeviceType::Modem => 8,
            DeviceType::Infiniband => 9,
            DeviceType::Bond => 10,
            DeviceType::Vlan => 11,
            DeviceType::Adsl => 12,
            DeviceType::Bridge => 13,
            DeviceType::Generic => 14,
            DeviceType::Team => 15,
            DeviceType::Tun => 16,
            DeviceType::IpTunnel => 17,
            DeviceType::Macvlan => 18,
            DeviceType::Vxlan => 19,
            DeviceType::Veth => 20,
            DeviceType::Macsec => 21,
            DeviceType::Dummy => 22,
            DeviceType::Ppp => 23,
            DeviceType::OvsInterface => 24,
            DeviceType::OvsPort => 25,
            DeviceType::OvsBridge => 26,
            DeviceType::Wpan => 27,
            DeviceType::SixLowpan => 28,
            DeviceType::Wireguard => 29,
            DeviceType::WifiP2p => 30,
            DeviceType::Vrf => 31,
        }
    }
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_str = match self {
            DeviceType::Unknown => "Unknown",
            DeviceType::Generic => "Generic",
            DeviceType::Ethernet => "Ethernet",
            DeviceType::Wifi => "Wi-Fi",
            DeviceType::Bluetooth => "Bluetooth",
            DeviceType::OlpcMesh => "OLPC Mesh",
            DeviceType::Wimax => "WiMAX",
            DeviceType::Modem => "Modem",
            DeviceType::Infiniband => "InfiniBand",
            DeviceType::Bond => "Bond",
            DeviceType::Vlan => "VLAN",
            DeviceType::Adsl => "ADSL",
            DeviceType::Bridge => "Bridge",
            DeviceType::Team => "Team",
            DeviceType::Tun => "TUN/TAP",
            DeviceType::IpTunnel => "IP Tunnel",
            DeviceType::Macvlan => "MACVLAN",
            DeviceType::Vxlan => "VXLAN",
            DeviceType::Veth => "VETH",
            DeviceType::Macsec => "MACsec",
            DeviceType::Dummy => "Dummy",
            DeviceType::Ppp => "PPP",
            DeviceType::OvsInterface => "Open vSwitch Interface",
            DeviceType::OvsPort => "Open vSwitch Port",
            DeviceType::OvsBridge => "Open vSwitch Bridge",
            DeviceType::Wpan => "WPAN",
            DeviceType::SixLowpan => "6LoWPAN",
            DeviceType::Wireguard => "WireGuard",
            DeviceType::WifiP2p => "Wi-Fi P2P",
            DeviceType::Vrf => "VRF",
        };
        write!(f, "{}", type_str)
    }
}