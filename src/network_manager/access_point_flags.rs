use bitflags::bitflags;

bitflags! {
    /// 802.11 access point flags.
    pub struct NM80211ApFlags: u32 {
        const NONE    = 0x00000000;
        const PRIVACY = 0x00000001;
        const WPS     = 0x00000002;
        const WPS_PBC = 0x00000004;
        const WPS_PIN = 0x00000008;
    }
}