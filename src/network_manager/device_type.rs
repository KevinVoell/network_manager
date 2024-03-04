#[derive(Debug, PartialEq, PartialOrd)]
enum DeviceType {
    Ethernet,
    WiFi,
    Bluetooth,
    OLPC,
    WiMAX,
    Modem,
    InfiniBand,
    Bond,
    VLAN,
    ADSL,
    Bridge,
    Generic,
    Team,
    TUN,
    IPTunnel,
    MACVLAN,
    VXLAN,
    Veth,
}

impl DeviceType {
    fn from_code(code: u32) -> Option<Self> {
        match code {
            1 => Some(DeviceType::Ethernet),
            2 => Some(DeviceType::WiFi),
            5 => Some(DeviceType::Bluetooth),
            6 => Some(DeviceType::OLPC),
            7 => Some(DeviceType::WiMAX),
            8 => Some(DeviceType::Modem),
            9 => Some(DeviceType::InfiniBand),
            10 => Some(DeviceType::Bond),
            11 => Some(DeviceType::VLAN),
            12 => Some(DeviceType::ADSL),
            13 => Some(DeviceType::Bridge),
            14 => Some(DeviceType::Generic),
            15 => Some(DeviceType::Team),
            16 => Some(DeviceType::TUN),
            17 => Some(DeviceType::IPTunnel),
            18 => Some(DeviceType::MACVLAN),
            19 => Some(DeviceType::VXLAN),
            20 => Some(DeviceType::Veth),
            _ => None,
        }
    }
}
