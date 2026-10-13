//! Macro management domain model.

use serde::{Deserialize, Serialize};
pub use crate::protocol::macro_upload::{DeviceMacro, MacroEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroItem {
    pub id: String,
    pub name: String,
    pub slot: u8,
    pub events: Vec<MacroEvent>,
    pub delay_ms: u32,
}

impl Default for MacroItem {
    fn default() -> Self {
        Self {
            id: "macro_1".into(),
            name: "Macro 1".into(),
            slot: 2,
            events: vec![
                MacroEvent::press(0x06), // 'C' key
                MacroEvent::release(0x06),
            ],
            delay_ms: 10,
        }
    }
}

impl MacroItem {
    pub fn to_device_macro(&self) -> DeviceMacro {
        DeviceMacro {
            name: self.name.clone(),
            slot: self.slot,
            events: self.events.clone(),
        }
    }
}
