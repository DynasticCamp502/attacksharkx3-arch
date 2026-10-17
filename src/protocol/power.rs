//! Report 0x05: Power management (sleep timers) and key debounce.

use crate::protocol::checksum::apply_checksum;
use crate::protocol::constants::*;
use anyhow::{bail, Result};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PowerConfig {
    /// Sleep idle time in minutes (0.5 .. 30.0)
    pub sleep_minutes: f32,
    /// Deep sleep idle time in minutes (1 .. 60)
    pub deep_sleep_minutes: u8,
    /// Key response time / debounce in milliseconds (2 .. 50)
    pub key_debounce_ms: u8,
}

impl Default for PowerConfig {
    fn default() -> Self {
        Self {
            sleep_minutes: 5.0,
            deep_sleep_minutes: 30,
            key_debounce_ms: 4,
        }
    }
}

/// Build 15-byte Report 0x05 packet
pub fn build_power_report(config: &PowerConfig) -> Result<[u8; LEN_REPORT_POWER]> {
    let sleep_raw = (config.sleep_minutes * 2.0).round() as u8;
    if !(1..=60).contains(&sleep_raw) {
        bail!("Sleep minutes {} must resolve to raw value 1..60", config.sleep_minutes);
    }
    if !(1..=60).contains(&config.deep_sleep_minutes) {
        bail!("Deep sleep minutes {} must be between 1 and 60", config.deep_sleep_minutes);
    }
    let debounce_raw = config.key_debounce_ms / 2;
    if !(1..=25).contains(&debounce_raw) {
        bail!("Debounce {} ms must be between 2 and 50 ms", config.key_debounce_ms);
    }

    let mut buf = [0u8; LEN_REPORT_POWER];
    buf[0] = REPORT_POWER;
    buf[1] = LEN_REPORT_POWER as u8;
    buf[2] = PROFILE_INDEX;

    buf[3] = 0x00;
    // Deep sleep split across high nibbles with 0x03 and 0x08 fixed low nibbles
    buf[4] = (config.deep_sleep_minutes & 0xF0) | 0x03;
    buf[5] = ((config.deep_sleep_minutes & 0x0F) << 4) | 0x08;
    buf[6] = 0x00;
    buf[7] = 0x00;
    buf[8] = 0xFF;
    buf[9] = sleep_raw;
    buf[10] = debounce_raw;

    // Bytes 11..13: 16-bit BE checksum
    apply_checksum(&mut buf, CHECK_OFFSET_POWER);

    Ok(buf)
}
