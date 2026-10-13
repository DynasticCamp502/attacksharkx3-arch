//! Report 0x09: Macro upload packets and 128-byte block formatting.

use crate::protocol::checksum::apply_macro_checksum;
use crate::protocol::constants::*;
use anyhow::{bail, Result};

/// A single key press or release event in a macro
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MacroEvent {
    pub is_release: bool,
    pub hid_usage: u8,
}

impl MacroEvent {
    pub fn press(hid_usage: u8) -> Self {
        Self { is_release: false, hid_usage }
    }

    pub fn release(hid_usage: u8) -> Self {
        Self { is_release: true, hid_usage }
    }

    pub fn to_bytes(&self) -> [u8; 2] {
        let flag = if self.is_release { 0x81 } else { 0x01 };
        [flag, self.hid_usage]
    }
}

/// A complete hardware macro ready for device upload
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DeviceMacro {
    pub name: String,
    pub slot: u8,
    pub events: Vec<MacroEvent>,
}

impl Default for DeviceMacro {
    fn default() -> Self {
        Self {
            name: "Default Macro".into(),
            slot: DEFAULT_MACRO_SLOT,
            events: Vec::new(),
        }
    }
}

impl DeviceMacro {
    /// Maximum number of events that fit in the 128-byte block (bytes 26..126 = 100 bytes / 2 = 50 events)
    pub const MAX_EVENTS: usize = 50;

    /// Build the 128-byte block
    pub fn build_block(&self) -> Result<[u8; MACRO_BLOCK_LEN]> {
        if self.events.len() > Self::MAX_EVENTS {
            bail!("Macro exceeds maximum event capacity ({} > {})", self.events.len(), Self::MAX_EVENTS);
        }
        if self.slot == 0 {
            bail!("Macro slot 0 is rejected by firmware (use slot 2)");
        }

        let mut block = [0u8; MACRO_BLOCK_LEN];
        block[0] = self.slot;
        // bytes 1..4: zero
        block[4] = 0x01; // constant in all captures
        // bytes 5..25: zero
        block[25] = self.events.len() as u8;

        for (i, event) in self.events.iter().enumerate() {
            let offset = 26 + i * 2;
            let bytes = event.to_bytes();
            block[offset] = bytes[0];
            block[offset + 1] = bytes[1];
        }

        apply_macro_checksum(&mut block);
        Ok(block)
    }

    /// Split the 128-byte block into 3 chunks of Report 0x09:
    /// Chunk 0: 64 bytes (header 4 + payload 60)
    /// Chunk 1: 64 bytes (header 4 + payload 60)
    /// Chunk 2: 12 bytes (header 4 + payload 8)
    pub fn build_chunks(&self) -> Result<([u8; 64], [u8; 64], [u8; 12])> {
        let block = self.build_block()?;

        // Chunk 0: 60 bytes of block (0..60)
        let mut chunk0 = [0u8; 64];
        chunk0[0] = REPORT_MACRO;
        chunk0[1] = 0x40; // 64 bytes
        chunk0[2] = MACRO_BLOCK_ID; // 0x08
        chunk0[3] = 0x00; // chunk index 0
        chunk0[4..64].copy_from_slice(&block[0..60]);

        // Chunk 1: 60 bytes of block (60..120)
        let mut chunk1 = [0u8; 64];
        chunk1[0] = REPORT_MACRO;
        chunk1[1] = 0x40; // 64 bytes
        chunk1[2] = MACRO_BLOCK_ID;
        chunk1[3] = 0x01; // chunk index 1
        chunk1[4..64].copy_from_slice(&block[60..120]);

        // Chunk 2: remaining 8 bytes of block (120..128)
        let mut chunk2 = [0u8; 12];
        chunk2[0] = REPORT_MACRO;
        chunk2[1] = 0x0C; // 12 bytes
        chunk2[2] = MACRO_BLOCK_ID;
        chunk2[3] = 0x02; // chunk index 2
        chunk2[4..12].copy_from_slice(&block[120..128]);

        Ok((chunk0, chunk1, chunk2))
    }
}
