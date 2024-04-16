use bitflags::bitflags;

bitflags! {
    /// 802.11 access point security flags.
    pub struct NM80211ApSecurityFlags: u32 {
        const NONE                     = 0x00000000;
        const PAIR_WEP40               = 0x00000001;
        const PAIR_WEP104              = 0x00000002;
        const PAIR_TKIP                = 0x00000004;
        const PAIR_CCMP                = 0x00000008;
        const GROUP_WEP40              = 0x00000010;
        const GROUP_WEP104             = 0x00000020;
        const GROUP_TKIP               = 0x00000040;
        const GROUP_CCMP               = 0x00000080;
        const KEY_MGMT_PSK             = 0x00000100;
        const KEY_MGMT_802_1X          = 0x00000200;
        const KEY_MGMT_SAE             = 0x00000400;
        const KEY_MGMT_OWE             = 0x00000800;
        const KEY_MGMT_OWE_TM          = 0x00001000;
        const KEY_MGMT_EAP_SUITE_B_192 = 0x00002000;
    }
}
