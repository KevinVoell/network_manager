//! Metered connection state enumeration for NetworkManager devices
//!
//! This module contains the `Metered` enum that represents whether a device's connection
//! is considered metered (data usage is limited/expensive).

/// NetworkManager metered connection states
/// 
/// These states correspond to the `NMMetered` enum in NetworkManager's D-Bus interface.
/// The numeric values match those defined in the NetworkManager source code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Metered {
    /// The metered status is unknown
    Unknown,
    /// Metered, the value was explicitly configured
    Yes,
    /// Not metered, the value was explicitly configured
    No,
    /// Metered, the value was guessed
    GuessYes,
    /// Not metered, the value was guessed
    GuessNo,
}

impl From<u32> for Metered {
    /// Convert a raw NetworkManager metered value to a typed enum
    fn from(value: u32) -> Self {
        match value {
            0 => Metered::Unknown,
            1 => Metered::Yes,
            2 => Metered::No,
            3 => Metered::GuessYes,
            4 => Metered::GuessNo,
            _ => Metered::Unknown,
        }
    }
}

impl From<Metered> for u32 {
    /// Convert a Metered enum value back to its raw NetworkManager value
    fn from(metered: Metered) -> Self {
        match metered {
            Metered::Unknown => 0,
            Metered::Yes => 1,
            Metered::No => 2,
            Metered::GuessYes => 3,
            Metered::GuessNo => 4,
        }
    }
}

impl std::fmt::Display for Metered {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let metered_str = match self {
            Metered::Unknown => "Unknown",
            Metered::Yes => "Metered",
            Metered::No => "Not Metered",
            Metered::GuessYes => "Probably Metered",
            Metered::GuessNo => "Probably Not Metered",
        };
        write!(f, "{}", metered_str)
    }
}

impl Metered {
    /// Returns true if the connection is metered (either explicitly or guessed)
    pub fn is_metered(&self) -> bool {
        matches!(self, Metered::Yes | Metered::GuessYes)
    }
    
    /// Returns true if the connection is not metered (either explicitly or guessed)
    pub fn is_not_metered(&self) -> bool {
        matches!(self, Metered::No | Metered::GuessNo)
    }
    
    /// Returns true if the metered status was explicitly configured
    pub fn is_explicit(&self) -> bool {
        matches!(self, Metered::Yes | Metered::No)
    }
    
    /// Returns true if the metered status was guessed
    pub fn is_guessed(&self) -> bool {
        matches!(self, Metered::GuessYes | Metered::GuessNo)
    }
}