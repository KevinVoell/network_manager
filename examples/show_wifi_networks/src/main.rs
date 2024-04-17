//! # show_wifi_networks Example
//!
//! Rust example that lists Wi-Fi access points NetworkManager scanned on Wi-Fi devices.
//! Usage: sudo ./show_wifi_networks
//!
//! This example is based off an official example written in Python available here:
//! https://gitlab.freedesktop.org/NetworkManager/NetworkManager/-/blob/main/examples/python/gi/show-wifi-networks.py
//!
//! DISCLAIMER:
//! The example code provided here is for illustrative purposes only. It is provided "AS IS",
//! without warranty of any kind, express or implied, including but not limited to the warranties of
//! merchantability, fitness for a particular purpose, and non-infringement. In no event shall the authors
//! or copyright holders be liable for any claim, damages, or other liability, whether in an action of
//! contract, tort, or otherwise, arising from, out of, or in connection with the example code or
//! the use or other dealings in the example code.
use network_manager::{
    AccessPointProxy, Channel, DeviceProxy, DeviceType, NM80211ApFlags, NM80211ApSecurityFlags,
    NetworkManagerProxy, WirelessProxy,
};
use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::io::Read;
use std::time::Duration;
use zbus::zvariant::OwnedObjectPath;
use zbus::Connection;

const SCAN_THRESHOLD_MSEC: u128 = 10_000;

#[tokio::main]
async fn main() {
    let connection = Connection::system()
        .await
        .expect("Could not get a connection.");

    let nm = NetworkManagerProxy::new(&connection)
        .await
        .expect("Could not get NetworkManager");

    let devices = nm.get_devices().await.expect("Could not get devices");

    let mut is_first = true;
    for device_path in devices {
        let device = DeviceProxy::new_from_path(device_path.clone(), &connection)
            .await
            .expect("Could not get device");

        let device_type = DeviceType::from_code(device.device_type().await.unwrap());

        match device_type {
            Some(x) => {
                if x != DeviceType::WiFi {
                    continue;
                }
            }
            None => continue,
        };

        if !is_first {
            println!("");
        } else {
            is_first = false;
        }

        let wireless_proxy = WirelessProxy::new_from_path(device_path, &connection)
            .await
            .expect("Could not get wireless device");

        device_ensure_scanned(&wireless_proxy).await;

        print_device_info(&wireless_proxy, &device, &connection).await;

        let access_points = wireless_proxy
            .get_access_points()
            .await
            .expect("Could not get access points");

        for access_point_path in access_points {
            let access_point =
                AccessPointProxy::new_from_path(access_point_path.clone(), &connection)
                    .await
                    .expect("Could not get access point");

            print_ap_info(&access_point, access_point_path).await;
        }
    }
}

async fn device_ensure_scanned(wireless_proxy: &WirelessProxy<'_>) {
    if !device_needs_scan(wireless_proxy).await {
        return;
    }

    let options = HashMap::new();
    wireless_proxy
        .request_scan(options)
        .await
        .expect("Could not request scan, try running as sudo.");

    println!("Scan requested, waiting for scan to complete");
    for _ in 0..5 {
        tokio::time::sleep(tokio::time::Duration::from_millis(10 * 1000)).await;
        if !device_needs_scan(wireless_proxy).await {
            return;
        }

        println!("Still waiting for scan to complete");
    }

    panic!("request scan failed");
}

fn get_uptime() -> Duration {
    let mut uptime_str = String::new();
    let mut f = File::open("/proc/uptime").expect("Failed to open /proc/uptime");
    f.read_to_string(&mut uptime_str)
        .expect("Failed to read /proc/uptime");

    let uptime_secs = uptime_str
        .split('.')
        .next()
        .expect("Malformed /proc/uptime")
        .parse::<u64>()
        .expect("Malformed /proc/uptime");
    Duration::from_secs(uptime_secs)
}

async fn device_needs_scan(wireless_proxy: &WirelessProxy<'_>) -> bool {
    let last_scan: u128 = wireless_proxy
        .last_scan()
        .await
        .expect("Could not get last scan")
        .try_into()
        .unwrap();

    let last_boot = get_uptime().as_millis();

    last_scan == 0 || last_scan < last_boot - SCAN_THRESHOLD_MSEC // TODO: Set scan threshold
}

async fn print_device_info(
    wireless_proxy: &WirelessProxy<'_>,
    device: &DeviceProxy<'_>,
    connection: &Connection,
) {
    let t: u128 = wireless_proxy
        .last_scan()
        .await
        .expect("Could not get last scan")
        .try_into()
        .unwrap();

    let mut last_scan: String;

    if t == 0 {
        last_scan = String::from("no scan completed");
    } else {
        let last_boot = get_uptime().as_millis();

        let duration = (last_boot - t) / 1000;
        last_scan = format!("{duration} sec ago");

        if device_needs_scan(wireless_proxy).await {
            last_scan += " (stale)";
        }
    }

    let ap = wireless_proxy.active_access_point().await;

    let active_ap = match ap {
        Ok(ap) => {
            let access_point = AccessPointProxy::new_from_path(ap.clone(), connection)
                .await
                .expect("Could not get access point");

            let ssid = access_point.ssid().await.expect("Could not get SSID");

            format!("{} ({})", String::from_utf8(ssid).unwrap(), ap)
        }
        Err(_) => "none".to_string(),
    };

    println!(
        "Device:       {}",
        device.interface().await.expect("Could not get interface")
    );
    println!(
        "D-Bus path:   {}",
        device.path().await.expect("Could not get path")
    );
    println!(
        "Driver:       {}",
        device.driver().await.expect("Could not get driver")
    );
    println!("Active AP:    {}", active_ap);
    println!("Last scan:    {}", last_scan);
}

async fn print_ap_info(access_point: &AccessPointProxy<'_>, access_point_path: OwnedObjectPath) {
    let strength = access_point
        .strength()
        .await
        .expect("Could not get strength");

    let frequency = access_point
        .frequency()
        .await
        .expect("Could not get frequency");

    let flags = access_point.flags().await.expect("Could not get flags");

    let wpa_flags = access_point
        .wpa_flags()
        .await
        .expect("Could not get wpa_flags");

    let rsn_flags = access_point
        .rsn_flags()
        .await
        .expect("Could not get rsn_flags");

    let mode = access_point.mode().await.expect("Could not get mode");

    let t = access_point
        .last_seen()
        .await
        .expect("Could not get last_seen");

    let last_seen = if t == 0 {
        "no scan completed".to_string()
    } else {
        let last_boot: i32 = get_uptime().as_millis().try_into().unwrap();

        let duration = (last_boot / 1000) - t;
        format!("{duration} sec ago")
    };

    println!("----------------------");
    println!("D-Bus path:       {}", access_point_path);
    println!(
        "SSID:             {}",
        String::from_utf8(access_point.ssid().await.unwrap()).unwrap()
    );
    // TODO: BSSID is missing
    //println!("BSSID\t\t{}", access_point.bssid().await.unwrap());

    println!("Last seen:        {}", last_seen);
    println!("Frequency:        {}", frequency);
    println!("Channel:          {}", wifi_freq_to_channel(frequency));
    println!("Mode:             {} ({})", mode_to_string(mode), mode);
    println!(
        "Flags             {}",
        flags_to_str(NM80211ApFlags::from_bits(flags).unwrap())
    );
    println!(
        "WPA flags:        {}",
        security_flags_to_str(NM80211ApSecurityFlags::from_bits(wpa_flags).unwrap())
    );
    println!(
        "RSN flags:        {}",
        security_flags_to_str(NM80211ApSecurityFlags::from_bits(rsn_flags).unwrap())
    );
    println!(
        "Security:         {}",
        ap_security_flags_to_security(
            NM80211ApFlags::from_bits(flags).unwrap(),
            NM80211ApSecurityFlags::from_bits(wpa_flags).unwrap(),
            NM80211ApSecurityFlags::from_bits(rsn_flags).unwrap()
        )
    );
    println!("Strength:         {}", wifi_strength_bars(strength));
}

fn mode_to_string(mode: u32) -> String {
    match mode {
        1 => "ADHOC",
        2 => "INFRA",
        3 => "AP",
        4 => "MESH",
        _ => "UNKNOWN",
    }
    .to_string()
}

fn ap_security_flags_to_security(
    flags: NM80211ApFlags,
    wpa_flags: NM80211ApSecurityFlags,
    rsn_flags: NM80211ApSecurityFlags,
) -> String {
    let mut str = String::new();

    if flags.contains(NM80211ApFlags::PRIVACY) && wpa_flags.is_empty() && rsn_flags.is_empty() {
        str = format!(" WEP");
    }

    if !wpa_flags.is_empty() {
        str = format!("{str} WPA1");
    }

    if !rsn_flags.is_empty() {
        str = format!("{str} WPA2");
    }

    if wpa_flags.contains(NM80211ApSecurityFlags::KEY_MGMT_802_1X)
        || rsn_flags.contains(NM80211ApSecurityFlags::KEY_MGMT_802_1X)
    {
        str = format!("{str} 802.1X")
    }

    str.trim_start().to_string()
}

fn wifi_strength_bars(strength: u8) -> String {
    if strength > 80 {
        return "****".to_string();
    }
    if strength > 55 {
        return "***".to_string();
    }
    if strength > 30 {
        return "**".to_string();
    }
    if strength > 5 {
        return "*".to_string();
    }

    "".to_string()
}

fn wifi_freq_to_channel(freq: u32) -> u32 {
    let mut i = 0;

    if freq > 4900 {
        let a_table = Channel::a_frequencies();
        while a_table[i].frequency != 0 && a_table[i].frequency != freq {
            i += 1;
        }

        return a_table[i].channel;
    }

    let bg_table = Channel::gb_frequencies();
    while bg_table[i].frequency != 0 && bg_table[i].frequency != freq {
        i += 1;
    }

    bg_table[i].channel
}

fn flags_to_str(flags: NM80211ApFlags) -> String {
    let mut flags_str = String::new();

    if flags.contains(NM80211ApFlags::PRIVACY) {
        _ = flags_str.write_str("Privacy|");
    }

    if flags.contains(NM80211ApFlags::WPS) {
        _ = flags_str.write_str("WPS|");
    }

    if flags.contains(NM80211ApFlags::WPS_PBC) {
        _ = flags_str.write_str("WPS_PBC|");
    }

    if flags.contains(NM80211ApFlags::WPS_PIN) {
        _ = flags_str.write_str("WPS_PIN");
    }

    if flags_str.is_empty() {
        _ = flags_str.write_str("None");
    }

    flags_str.trim_end_matches('|').to_string()
}

fn security_flags_to_str(flags: NM80211ApSecurityFlags) -> String {
    let mut flags_str = String::new();

    if flags.contains(NM80211ApSecurityFlags::PAIR_WEP40) {
        _ = flags_str.write_str("PAIR_WEP40|");
    }

    if flags.contains(NM80211ApSecurityFlags::PAIR_WEP104) {
        _ = flags_str.write_str("PAIR_WEP104|");
    }

    if flags.contains(NM80211ApSecurityFlags::PAIR_TKIP) {
        _ = flags_str.write_str("PAIR_TKIP|");
    }

    if flags.contains(NM80211ApSecurityFlags::PAIR_CCMP) {
        _ = flags_str.write_str("PAIR_CCMP|");
    }

    if flags.contains(NM80211ApSecurityFlags::GROUP_WEP40) {
        _ = flags_str.write_str("GROUP_WEP40|");
    }

    if flags.contains(NM80211ApSecurityFlags::GROUP_WEP104) {
        _ = flags_str.write_str("GROUP_WEP104|");
    }

    if flags.contains(NM80211ApSecurityFlags::GROUP_TKIP) {
        _ = flags_str.write_str("GROUP_TKIP|");
    }

    if flags.contains(NM80211ApSecurityFlags::GROUP_CCMP) {
        _ = flags_str.write_str("GROUP_CCMP|");
    }

    if flags.contains(NM80211ApSecurityFlags::KEY_MGMT_PSK) {
        _ = flags_str.write_str("KEY_MGMT_PSK|");
    }

    if flags.contains(NM80211ApSecurityFlags::KEY_MGMT_802_1X) {
        _ = flags_str.write_str("KEY_MGMT_802_1X|");
    }

    if flags.contains(NM80211ApSecurityFlags::KEY_MGMT_SAE) {
        _ = flags_str.write_str("KEY_MGMT_SAE|");
    }

    if flags.contains(NM80211ApSecurityFlags::KEY_MGMT_OWE) {
        _ = flags_str.write_str("KEY_MGMT_OWE|");
    }

    if flags.contains(NM80211ApSecurityFlags::KEY_MGMT_OWE_TM) {
        _ = flags_str.write_str("KEY_MGMT_OWE_TM|");
    }

    if flags.contains(NM80211ApSecurityFlags::KEY_MGMT_EAP_SUITE_B_192) {
        _ = flags_str.write_str("KEY_MGMT_EAP_SUITE_B_192|");
    }

    if flags_str.is_empty() {
        _ = flags_str.write_str("None");
    }

    flags_str.trim_end_matches('|').to_string()
}
