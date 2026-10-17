//! Battery status decoding and voltage discharge curve approximation.

use crate::protocol::constants::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatteryStatus {
    pub raw_byte: u8,
    pub voltage: f32,
    pub percentage: u8,
    pub is_charging: bool,
}

impl BatteryStatus {
    /// Parse the 5-byte input report: `03 10 [voltage] 01 09`
    pub fn parse(buf: &[u8]) -> Option<Self> {
        if buf.len() < 5 {
            return None;
        }
        if buf[0] != STATUS_REPORT_ID || buf[1] != STATUS_KIND_BATTERY {
            return None;
        }
        let raw_byte = buf[2];
        let flags = buf[3];
        let is_charging = flags & 0x02 != 0; // or charging flag

        let voltage = (raw_byte as f32) / 16.0;
        let percentage = voltage_to_percentage(voltage);

        Some(Self {
            raw_byte,
            voltage,
            percentage,
            is_charging,
        })
    }
}

/// Convert battery voltage (lithium cell ~3.4V to ~4.2V) to percentage 0..100%
pub fn voltage_to_percentage(voltage: f32) -> u8 {
    if voltage >= 4.18 {
        100
    } else if voltage >= 4.08 {
        90 + ((voltage - 4.08) / 0.10 * 10.0) as u8
    } else if voltage >= 3.98 {
        75 + ((voltage - 3.98) / 0.10 * 15.0) as u8
    } else if voltage >= 3.88 {
        60 + ((voltage - 3.88) / 0.10 * 15.0) as u8
    } else if voltage >= 3.78 {
        40 + ((voltage - 3.78) / 0.10 * 20.0) as u8
    } else if voltage >= 3.68 {
        20 + ((voltage - 3.68) / 0.10 * 20.0) as u8
    } else if voltage >= 3.55 {
        5 + ((voltage - 3.55) / 0.13 * 15.0) as u8
    } else if voltage >= 3.40 {
        ((voltage - 3.40) / 0.15 * 5.0) as u8
    } else {
        0
    }
}
