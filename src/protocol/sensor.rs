//! Report 0x04: Sensor settings and DPI stage table.

use crate::protocol::checksum::apply_checksum;
use crate::protocol::constants::*;
use anyhow::{bail, Result};

/// Convert human DPI (50..26000) to wire raw value (0..519): `raw = dpi / 50 - 1`.
pub fn dpi_to_raw(dpi: u32) -> Result<u16> {
    if !(DPI_MIN..=DPI_MAX).contains(&dpi) {
        bail!("DPI {} outside valid range {}..{}", dpi, DPI_MIN, DPI_MAX);
    }
    if dpi % DPI_STEP != 0 {
        bail!("DPI {} must be a multiple of {}", dpi, DPI_STEP);
    }
    Ok(((dpi / DPI_STEP) - 1) as u16)
}

/// Convert wire raw value to human DPI: `dpi = (raw + 1) * 50`.
pub fn raw_to_dpi(raw: u16) -> u32 {
    ((raw as u32) + 1) * DPI_STEP
}

/// Sensor and DPI configuration payload
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SensorConfig {
    pub dpi_stages: Vec<u32>,
    pub active_stage: usize,
    pub lod: LiftOffDistance,
    pub ripple_control: bool,
    pub angle_snap: bool,
    pub motion_sync: bool,
    pub colors: Vec<[u8; 3]>,
    pub enabled_mask: Option<u8>,
}

impl Default for SensorConfig {
    fn default() -> Self {
        Self {
            dpi_stages: vec![800, 1600, 2400, 3200, 5000, 26000],
            active_stage: 0, // 800 DPI default
            lod: LiftOffDistance::TwoMm,
            ripple_control: false,
            angle_snap: false,
            motion_sync: false,
            colors: vec![
                [255, 0, 0],     // Red
                [0, 255, 0],     // Green
                [0, 0, 255],     // Blue
                [255, 255, 0],   // Yellow
                [0, 255, 255],   // Cyan
                [255, 0, 255],   // Magenta
                [255, 64, 0],    // Orange
                [255, 255, 255], // White
            ],
            enabled_mask: Some(0x3F), // 6 active stages default
        }
    }
}

/// Build 56-byte Report 0x04 packet
pub fn build_sensor_report(config: &SensorConfig) -> Result<[u8; LEN_REPORT_SENSOR]> {
    let mut buf = [0u8; LEN_REPORT_SENSOR];
    buf[0] = REPORT_SENSOR;
    buf[1] = LEN_REPORT_SENSOR as u8;
    buf[2] = PROFILE_INDEX;

    // Byte 3: LOD (0x00 = 1mm, 0x01 = 2mm)
    buf[3] = config.lod as u8;
    // Byte 4: Ripple control
    buf[4] = if config.ripple_control { 1 } else { 0 };
    // Byte 5: Enabled DPI stage mask
    let mask = config.enabled_mask.unwrap_or_else(|| {
        let count = config.dpi_stages.len().min(DPI_SLOTS);
        ((1 << count) - 1) as u8
    });
    buf[5] = mask;
    // Byte 6: Angle snap
    buf[6] = if config.angle_snap { 1 } else { 0 };
    // Byte 7: Motion sync
    buf[7] = if config.motion_sync { 1 } else { 0 };

    // Bytes 8..16: Low bytes of DPI stages (slots 0..7)
    // Bytes 16..24: High bytes of DPI stages (slots 0..7)
    for (i, &dpi) in config.dpi_stages.iter().take(DPI_SLOTS).enumerate() {
        let raw = dpi_to_raw(dpi)?;
        buf[8 + i] = (raw & 0xFF) as u8;
        buf[16 + i] = ((raw >> 8) & 0xFF) as u8;
    }

    // Byte 24: Active stage (1-based on the wire)
    buf[24] = ((config.active_stage % config.dpi_stages.len()) + 1) as u8;

    // Bytes 25..49: 8 x RGB indicator colors
    for (i, rgb) in config.colors.iter().take(DPI_SLOTS).enumerate() {
        buf[25 + i * 3] = rgb[0];
        buf[26 + i * 3] = rgb[1];
        buf[27 + i * 3] = rgb[2];
    }

    // Byte 49: Constant 0x01 in all observed traffic
    buf[49] = 0x01;

    // Bytes 50..52: 16-bit BE checksum
    apply_checksum(&mut buf, CHECK_OFFSET_SENSOR);

    Ok(buf)
}
