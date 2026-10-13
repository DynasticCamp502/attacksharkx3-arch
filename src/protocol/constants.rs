//! USB and HID Protocol constants for Attack Shark X3 (Beken 2.4G OEM family).

pub const VENDOR_ID: u16 = 0x1D57;
pub const PRODUCT_ID_24G: u16 = 0xFA60;
pub const PRODUCT_ID_WIRED: u16 = 0xFA61;

/// Configuration collection usage page (MI_02 COL04)
pub const CONFIG_USAGE_PAGE: u16 = 0x000B;
pub const CONFIG_USAGE: u16 = 0x0000;

/// Battery / status collection usage page (MI_02 COL03)
pub const STATUS_USAGE_PAGE: u16 = 0x000A;
pub const STATUS_REPORT_ID: u8 = 0x03;
pub const STATUS_KIND_BATTERY: u8 = 0x10;

/// Report IDs
pub const REPORT_SENSOR: u8 = 0x04;
pub const REPORT_POWER: u8 = 0x05;
pub const REPORT_POLLING: u8 = 0x06;
pub const REPORT_BUTTONS: u8 = 0x08;
pub const REPORT_MACRO: u8 = 0x09;
pub const REPORT_COMMIT: u8 = 0x0C;

/// Report payload lengths (in bytes, including Report ID)
pub const LEN_REPORT_SENSOR: usize = 56;
pub const LEN_REPORT_POWER: usize = 15;
pub const LEN_REPORT_POLLING: usize = 9;
pub const LEN_REPORT_BUTTONS: usize = 59;
pub const LEN_REPORT_COMMIT: usize = 10;

/// Macro block sizing
pub const MACRO_BLOCK_LEN: usize = 128;
pub const MACRO_CHUNK_LEN: usize = 64;
pub const MACRO_FINAL_CHUNK_LEN: usize = 12;
pub const MACRO_BLOCK_ID: u8 = 0x08;
/// Dedicated firmware playback slot (slot 0 is ignored by firmware)
pub const DEFAULT_MACRO_SLOT: u8 = 0x02;

/// Checksum field byte offsets inside config packets
pub const CHECK_OFFSET_SENSOR: usize = 50;
pub const CHECK_OFFSET_POWER: usize = 11;
pub const CHECK_OFFSET_BUTTONS: usize = 57;

pub const PROFILE_INDEX: u8 = 0x01;

/// Sensor limits
pub const DPI_MIN: u32 = 50;
pub const DPI_MAX: u32 = 26000;
pub const DPI_STEP: u32 = 50;
pub const DPI_SLOTS: usize = 8;
pub const DEFAULT_ENABLED_STAGES: usize = 6;

/// Lift-off distance options
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum LiftOffDistance {
    OneMm = 0x00,
    TwoMm = 0x01,
}

impl Default for LiftOffDistance {
    fn default() -> Self {
        Self::TwoMm
    }
}

/// Polling rate options in Hz
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub enum PollingRate {
    Hz125 = 125,
    Hz250 = 250,
    Hz500 = 500,
    Hz1000 = 1000,
}

impl Default for PollingRate {
    fn default() -> Self {
        Self::Hz1000
    }
}

impl PollingRate {
    pub fn wire_code(&self) -> u8 {
        match self {
            Self::Hz1000 => 0x01,
            Self::Hz500 => 0x02,
            Self::Hz250 => 0x04,
            Self::Hz125 => 0x08,
        }
    }

    pub fn from_wire_code(code: u8) -> Option<Self> {
        match code {
            0x01 => Some(Self::Hz1000),
            0x02 => Some(Self::Hz500),
            0x04 => Some(Self::Hz250),
            0x08 => Some(Self::Hz125),
            _ => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Hz1000 => "1000 Hz (E-sports)",
            Self::Hz500 => "500 Hz (Gaming)",
            Self::Hz250 => "250 Hz (Office)",
            Self::Hz125 => "125 Hz (Power Saving)",
        }
    }
}
