//! Typed connection settings for NetworkManager
//!
//! This module provides typed structs for creating NetworkManager connection profiles
//! instead of working with raw nested HashMaps.

use std::collections::HashMap;
use zbus::zvariant::Value;

use super::metered::Metered;

/// Main connection configuration
#[derive(Debug, Clone)]
pub struct ConnectionSettings {
    /// Connection identifier/name
    pub id: String,
    /// Connection type (ethernet, wifi, etc.)
    pub connection_type: ConnectionType,
    /// Whether to autoconnect
    pub autoconnect: Option<bool>,
    /// UUID for the connection (auto-generated if None)
    pub uuid: Option<String>,
    /// Interface name (optional)
    pub interface_name: Option<String>,
    /// Whether the connection is metered
    pub metered: Option<Metered>,
    /// Connection zone for firewall
    pub zone: Option<String>,
}

/// Connection types supported by NetworkManager
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionType {
    Ethernet,
    Wireless,
    Bluetooth,
    Bond,
    Bridge,
    Vlan,
    Vpn,
    Generic,
}

impl std::fmt::Display for ConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_str = match self {
            ConnectionType::Ethernet => "802-3-ethernet",
            ConnectionType::Wireless => "802-11-wireless",
            ConnectionType::Bluetooth => "bluetooth",
            ConnectionType::Bond => "bond",
            ConnectionType::Bridge => "bridge",
            ConnectionType::Vlan => "vlan",
            ConnectionType::Vpn => "vpn",
            ConnectionType::Generic => "generic",
        };
        write!(f, "{}", type_str)
    }
}

/// Wireless (802.11) connection settings
#[derive(Debug, Clone)]
pub struct WirelessSettings {
    /// SSID (network name)
    pub ssid: Vec<u8>,
    /// Wireless mode
    pub mode: WirelessMode,
    /// MAC address of the wireless device to use
    pub mac_address: Option<String>,
    /// Hidden network
    pub hidden: Option<bool>,
    /// Band preference
    pub band: Option<WirelessBand>,
    /// Channel number (optional)
    pub channel: Option<u32>,
}

/// Wireless modes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WirelessMode {
    Infrastructure,
    Adhoc,
    Ap,
    Mesh,
}

impl std::fmt::Display for WirelessMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode_str = match self {
            WirelessMode::Infrastructure => "infrastructure",
            WirelessMode::Adhoc => "adhoc",
            WirelessMode::Ap => "ap",
            WirelessMode::Mesh => "mesh",
        };
        write!(f, "{}", mode_str)
    }
}

/// Wireless band preferences
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WirelessBand {
    /// 2.4 GHz band
    Bg,
    /// 5 GHz band
    A,
}

impl std::fmt::Display for WirelessBand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let band_str = match self {
            WirelessBand::Bg => "bg",
            WirelessBand::A => "a",
        };
        write!(f, "{}", band_str)
    }
}

/// Wireless security settings
#[derive(Debug, Clone)]
pub struct WirelessSecuritySettings {
    /// Key management method
    pub key_mgmt: KeyManagement,
    /// Pre-shared key (for WPA-PSK)
    pub psk: Option<String>,
    /// WEP key (for WEP)
    pub wep_key0: Option<String>,
    /// WEP key type
    pub wep_key_type: Option<WepKeyType>,
    /// Authentication algorithm
    pub auth_alg: Option<AuthAlgorithm>,
}

/// Key management methods
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyManagement {
    None,
    WpaPsk,
    WpaEap,
    Wep,
    Ieee8021x,
    WpaPskSha256,
    WpaEapSha256,
    Sae,
    OweTransition,
    Owe,
}

impl std::fmt::Display for KeyManagement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mgmt_str = match self {
            KeyManagement::None => "none",
            KeyManagement::WpaPsk => "wpa-psk",
            KeyManagement::WpaEap => "wpa-eap",
            KeyManagement::Wep => "wep",
            KeyManagement::Ieee8021x => "ieee8021x",
            KeyManagement::WpaPskSha256 => "wpa-psk-sha256",
            KeyManagement::WpaEapSha256 => "wpa-eap-sha256",
            KeyManagement::Sae => "sae",
            KeyManagement::OweTransition => "owe-transition",
            KeyManagement::Owe => "owe",
        };
        write!(f, "{}", mgmt_str)
    }
}

/// WEP key types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WepKeyType {
    Key,
    Passphrase,
}

impl std::fmt::Display for WepKeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_str = match self {
            WepKeyType::Key => "key",
            WepKeyType::Passphrase => "passphrase",
        };
        write!(f, "{}", type_str)
    }
}

/// Authentication algorithms
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthAlgorithm {
    Open,
    Shared,
    Leap,
}

impl std::fmt::Display for AuthAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let alg_str = match self {
            AuthAlgorithm::Open => "open",
            AuthAlgorithm::Shared => "shared",
            AuthAlgorithm::Leap => "leap",
        };
        write!(f, "{}", alg_str)
    }
}

/// IP configuration settings (IPv4 or IPv6)
#[derive(Debug, Clone)]
pub struct IpSettings {
    /// IP configuration method
    pub method: IpMethod,
    /// Static IP addresses (for manual method)
    pub addresses: Option<Vec<IpAddress>>,
    /// DNS servers
    pub dns: Option<Vec<String>>,
    /// DNS search domains
    pub dns_search: Option<Vec<String>>,
    /// Routes
    pub routes: Option<Vec<IpRoute>>,
    /// Whether to ignore auto-routes
    pub ignore_auto_routes: Option<bool>,
    /// Whether to ignore auto-DNS
    pub ignore_auto_dns: Option<bool>,
}

/// IP configuration methods
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpMethod {
    /// Automatic configuration (DHCP for IPv4, auto for IPv6)
    Auto,
    /// Manual (static) configuration
    Manual,
    /// Use link-local addressing only
    LinkLocal,
    /// Share this connection to other computers
    Shared,
    /// Disabled (no IP configuration)
    Disabled,
}

impl std::fmt::Display for IpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_str = match self {
            IpMethod::Auto => "auto",
            IpMethod::Manual => "manual",
            IpMethod::LinkLocal => "link-local",
            IpMethod::Shared => "shared",
            IpMethod::Disabled => "disabled",
        };
        write!(f, "{}", method_str)
    }
}

/// IP address configuration
#[derive(Debug, Clone)]
pub struct IpAddress {
    /// IP address
    pub address: String,
    /// Prefix length (subnet mask)
    pub prefix: u8,
    /// Gateway (optional)
    pub gateway: Option<String>,
}

/// IP route configuration
#[derive(Debug, Clone)]
pub struct IpRoute {
    /// Destination network
    pub dest: String,
    /// Prefix length
    pub prefix: u8,
    /// Next hop gateway
    pub next_hop: Option<String>,
    /// Route metric
    pub metric: Option<u32>,
}

/// Complete connection profile
#[derive(Debug, Clone)]
pub struct ConnectionProfile {
    /// Basic connection settings
    pub connection: ConnectionSettings,
    /// Wireless settings (if applicable)
    pub wireless: Option<WirelessSettings>,
    /// Wireless security settings (if applicable)
    pub wireless_security: Option<WirelessSecuritySettings>,
    /// IPv4 settings
    pub ipv4: Option<IpSettings>,
    /// IPv6 settings
    pub ipv6: Option<IpSettings>,
}

impl ConnectionProfile {
    /// Create a new Ethernet connection profile
    pub fn new_ethernet(id: String) -> Self {
        Self {
            connection: ConnectionSettings {
                id,
                connection_type: ConnectionType::Ethernet,
                autoconnect: Some(true),
                uuid: None,
                interface_name: None,
                metered: None,
                zone: None,
            },
            wireless: None,
            wireless_security: None,
            ipv4: Some(IpSettings {
                method: IpMethod::Auto,
                addresses: None,
                dns: None,
                dns_search: None,
                routes: None,
                ignore_auto_routes: None,
                ignore_auto_dns: None,
            }),
            ipv6: Some(IpSettings {
                method: IpMethod::Auto,
                addresses: None,
                dns: None,
                dns_search: None,
                routes: None,
                ignore_auto_routes: None,
                ignore_auto_dns: None,
            }),
        }
    }

    /// Create a new WPA2 WiFi connection profile
    pub fn new_wifi_wpa2(id: String, ssid: String, password: String) -> Self {
        Self {
            connection: ConnectionSettings {
                id,
                connection_type: ConnectionType::Wireless,
                autoconnect: Some(true),
                uuid: None,
                interface_name: None,
                metered: None,
                zone: None,
            },
            wireless: Some(WirelessSettings {
                ssid: ssid.into_bytes(),
                mode: WirelessMode::Infrastructure,
                mac_address: None,
                hidden: None,
                band: None,
                channel: None,
            }),
            wireless_security: Some(WirelessSecuritySettings {
                key_mgmt: KeyManagement::WpaPsk,
                psk: Some(password),
                wep_key0: None,
                wep_key_type: None,
                auth_alg: None,
            }),
            ipv4: Some(IpSettings {
                method: IpMethod::Auto,
                addresses: None,
                dns: None,
                dns_search: None,
                routes: None,
                ignore_auto_routes: None,
                ignore_auto_dns: None,
            }),
            ipv6: Some(IpSettings {
                method: IpMethod::Auto,
                addresses: None,
                dns: None,
                dns_search: None,
                routes: None,
                ignore_auto_routes: None,
                ignore_auto_dns: None,
            }),
        }
    }

    /// Create a new open WiFi connection profile (no security)
    pub fn new_wifi_open(id: String, ssid: String) -> Self {
        Self {
            connection: ConnectionSettings {
                id,
                connection_type: ConnectionType::Wireless,
                autoconnect: Some(true),
                uuid: None,
                interface_name: None,
                metered: None,
                zone: None,
            },
            wireless: Some(WirelessSettings {
                ssid: ssid.into_bytes(),
                mode: WirelessMode::Infrastructure,
                mac_address: None,
                hidden: None,
                band: None,
                channel: None,
            }),
            wireless_security: Some(WirelessSecuritySettings {
                key_mgmt: KeyManagement::None,
                psk: None,
                wep_key0: None,
                wep_key_type: None,
                auth_alg: None,
            }),
            ipv4: Some(IpSettings {
                method: IpMethod::Auto,
                addresses: None,
                dns: None,
                dns_search: None,
                routes: None,
                ignore_auto_routes: None,
                ignore_auto_dns: None,
            }),
            ipv6: Some(IpSettings {
                method: IpMethod::Auto,
                addresses: None,
                dns: None,
                dns_search: None,
                routes: None,
                ignore_auto_routes: None,
                ignore_auto_dns: None,
            }),
        }
    }
}

/// Convert ConnectionProfile to the HashMap format expected by NetworkManager
impl From<ConnectionProfile> for HashMap<String, HashMap<String, Value<'static>>> {
    fn from(profile: ConnectionProfile) -> Self {
        let mut settings = HashMap::new();

        // Connection settings
        let mut connection_map = HashMap::new();
        connection_map.insert("id".to_string(), Value::new(profile.connection.id));
        connection_map.insert("type".to_string(), Value::new(profile.connection.connection_type.to_string()));
        
        if let Some(autoconnect) = profile.connection.autoconnect {
            connection_map.insert("autoconnect".to_string(), Value::new(autoconnect));
        }
        
        if let Some(uuid) = profile.connection.uuid {
            connection_map.insert("uuid".to_string(), Value::new(uuid));
        }
        
        if let Some(interface_name) = profile.connection.interface_name {
            connection_map.insert("interface-name".to_string(), Value::new(interface_name));
        }
        
        if let Some(metered) = profile.connection.metered {
            connection_map.insert("metered".to_string(), Value::new(u32::from(metered)));
        }
        
        if let Some(zone) = profile.connection.zone {
            connection_map.insert("zone".to_string(), Value::new(zone));
        }
        
        settings.insert("connection".to_string(), connection_map);

        // Wireless settings
        if let Some(wireless) = profile.wireless {
            let mut wireless_map = HashMap::new();
            wireless_map.insert("ssid".to_string(), Value::new(wireless.ssid));
            wireless_map.insert("mode".to_string(), Value::new(wireless.mode.to_string()));
            
            if let Some(mac_address) = wireless.mac_address {
                wireless_map.insert("mac-address".to_string(), Value::new(mac_address));
            }
            
            if let Some(hidden) = wireless.hidden {
                wireless_map.insert("hidden".to_string(), Value::new(hidden));
            }
            
            if let Some(band) = wireless.band {
                wireless_map.insert("band".to_string(), Value::new(band.to_string()));
            }
            
            if let Some(channel) = wireless.channel {
                wireless_map.insert("channel".to_string(), Value::new(channel));
            }
            
            settings.insert("802-11-wireless".to_string(), wireless_map);
        }

        // Wireless security settings
        if let Some(security) = profile.wireless_security {
            let mut security_map = HashMap::new();
            security_map.insert("key-mgmt".to_string(), Value::new(security.key_mgmt.to_string()));
            
            if let Some(psk) = security.psk {
                security_map.insert("psk".to_string(), Value::new(psk));
            }
            
            if let Some(wep_key) = security.wep_key0 {
                security_map.insert("wep-key0".to_string(), Value::new(wep_key));
            }
            
            if let Some(wep_key_type) = security.wep_key_type {
                security_map.insert("wep-key-type".to_string(), Value::new(wep_key_type.to_string()));
            }
            
            if let Some(auth_alg) = security.auth_alg {
                security_map.insert("auth-alg".to_string(), Value::new(auth_alg.to_string()));
            }
            
            settings.insert("802-11-wireless-security".to_string(), security_map);
        }

        // IPv4 settings
        if let Some(ipv4) = profile.ipv4 {
            let mut ipv4_map = HashMap::new();
            ipv4_map.insert("method".to_string(), Value::new(ipv4.method.to_string()));
            
            if let Some(addresses) = ipv4.addresses {
                let addr_data: Vec<HashMap<String, Value<'static>>> = addresses.into_iter().map(|addr| {
                    let mut addr_map = HashMap::new();
                    addr_map.insert("address".to_string(), Value::new(addr.address));
                    addr_map.insert("prefix".to_string(), Value::new(addr.prefix as u32));
                    if let Some(gateway) = addr.gateway {
                        addr_map.insert("gateway".to_string(), Value::new(gateway));
                    }
                    addr_map
                }).collect();
                ipv4_map.insert("address-data".to_string(), Value::new(addr_data));
            }
            
            if let Some(dns) = ipv4.dns {
                ipv4_map.insert("dns".to_string(), Value::new(dns));
            }
            
            if let Some(dns_search) = ipv4.dns_search {
                ipv4_map.insert("dns-search".to_string(), Value::new(dns_search));
            }
            
            if let Some(ignore_auto_routes) = ipv4.ignore_auto_routes {
                ipv4_map.insert("ignore-auto-routes".to_string(), Value::new(ignore_auto_routes));
            }
            
            if let Some(ignore_auto_dns) = ipv4.ignore_auto_dns {
                ipv4_map.insert("ignore-auto-dns".to_string(), Value::new(ignore_auto_dns));
            }
            
            settings.insert("ipv4".to_string(), ipv4_map);
        }

        // IPv6 settings
        if let Some(ipv6) = profile.ipv6 {
            let mut ipv6_map = HashMap::new();
            ipv6_map.insert("method".to_string(), Value::new(ipv6.method.to_string()));
            
            if let Some(addresses) = ipv6.addresses {
                let addr_data: Vec<HashMap<String, Value<'static>>> = addresses.into_iter().map(|addr| {
                    let mut addr_map = HashMap::new();
                    addr_map.insert("address".to_string(), Value::new(addr.address));
                    addr_map.insert("prefix".to_string(), Value::new(addr.prefix as u32));
                    if let Some(gateway) = addr.gateway {
                        addr_map.insert("gateway".to_string(), Value::new(gateway));
                    }
                    addr_map
                }).collect();
                ipv6_map.insert("address-data".to_string(), Value::new(addr_data));
            }
            
            if let Some(dns) = ipv6.dns {
                ipv6_map.insert("dns".to_string(), Value::new(dns));
            }
            
            if let Some(dns_search) = ipv6.dns_search {
                ipv6_map.insert("dns-search".to_string(), Value::new(dns_search));
            }
            
            if let Some(ignore_auto_routes) = ipv6.ignore_auto_routes {
                ipv6_map.insert("ignore-auto-routes".to_string(), Value::new(ignore_auto_routes));
            }
            
            if let Some(ignore_auto_dns) = ipv6.ignore_auto_dns {
                ipv6_map.insert("ignore-auto-dns".to_string(), Value::new(ignore_auto_dns));
            }
            
            settings.insert("ipv6".to_string(), ipv6_map);
        }

        settings
    }
}