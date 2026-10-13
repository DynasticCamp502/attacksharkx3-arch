//! Report 0x08: 18-slot button table and action codes.

use crate::protocol::checksum::apply_checksum;
use crate::protocol::constants::*;
use anyhow::{bail, Result};

pub const NUM_SLOTS: usize = 18;

/// Physical button numbers mapped to 18-slot table
pub const BTN_LEFT_SLOT: usize = 0;
pub const BTN_RIGHT_SLOT: usize = 1;
pub const BTN_MIDDLE_SLOT: usize = 2;
pub const BTN_FORWARD_SLOT: usize = 6;
pub const BTN_BACKWARD_SLOT: usize = 7;

/// Button action types
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ButtonAction {
    ButtonOff,
    LeftClick,
    RightClick,
    MiddleClick,
    Forward,
    Backward,
    DoubleClick,
    FireButton,
    ScrollUp,
    ScrollDown,
    DpiCycle,
    DpiUp,
    DpiDown,
    EasyAim,
    KeyboardShortcut { modifiers: u8, key: u8 },
    Macro { slot: u8 },
    MediaPlayer,
    NextTrack,
    PlayPause,
    Mute,
    VolumeUp,
    MyComputer,
    BrowserHome,
    Raw(u8, u8, u8),
}

impl ButtonAction {
    pub fn to_bytes(&self) -> [u8; 3] {
        match *self {
            Self::ButtonOff => [0x01, 0, 0],
            Self::LeftClick => [0x02, 0, 0],
            Self::RightClick => [0x03, 0, 0],
            Self::MiddleClick => [0x04, 0, 0],
            Self::Backward => [0x05, 0, 0],
            Self::Forward => [0x06, 0, 0],
            Self::DoubleClick => [0x07, 0, 0],
            Self::FireButton => [0x08, 0, 0],
            Self::ScrollUp => [0x09, 0, 0],
            Self::ScrollDown => [0x0A, 0, 0],
            Self::DpiCycle => [0x0D, 0, 0],
            Self::DpiUp => [0x0E, 0, 0],
            Self::DpiDown => [0x0F, 0, 0],
            Self::EasyAim => [0x10, 0, 0],
            Self::KeyboardShortcut { modifiers, key } => [0x11, modifiers, key],
            Self::Macro { slot } => [0x12, 0x00, slot],
            Self::MediaPlayer => [0x15, 0, 0],
            Self::NextTrack => [0x17, 0, 0],
            Self::PlayPause => [0x18, 0, 0],
            Self::Mute => [0x1A, 0, 0],
            Self::VolumeUp => [0x1B, 0, 0],
            Self::MyComputer => [0x23, 0, 0],
            Self::BrowserHome => [0x25, 0, 0],
            Self::Raw(a, b, c) => [a, b, c],
        }
    }

    pub fn from_bytes(bytes: [u8; 3]) -> Self {
        match bytes[0] {
            0x01 => Self::ButtonOff,
            0x02 => Self::LeftClick,
            0x03 => Self::RightClick,
            0x04 => Self::MiddleClick,
            0x05 => Self::Backward,
            0x06 => Self::Forward,
            0x07 => Self::DoubleClick,
            0x08 => Self::FireButton,
            0x09 => Self::ScrollUp,
            0x0A => Self::ScrollDown,
            0x0D => Self::DpiCycle,
            0x0E => Self::DpiUp,
            0x0F => Self::DpiDown,
            0x10 => Self::EasyAim,
            0x11 => Self::KeyboardShortcut {
                modifiers: bytes[1],
                key: bytes[2],
            },
            0x12 => Self::Macro { slot: bytes[2] },
            0x15 => Self::MediaPlayer,
            0x17 => Self::NextTrack,
            0x18 => Self::PlayPause,
            0x1A => Self::Mute,
            0x1B => Self::VolumeUp,
            0x23 => Self::MyComputer,
            0x25 => Self::BrowserHome,
            other => Self::Raw(other, bytes[1], bytes[2]),
        }
    }

    pub fn label(&self) -> String {
        match self {
            Self::ButtonOff => "Disabled".into(),
            Self::LeftClick => "Left Click".into(),
            Self::RightClick => "Right Click".into(),
            Self::MiddleClick => "Middle Click".into(),
            Self::Forward => "Forward".into(),
            Self::Backward => "Backward".into(),
            Self::DoubleClick => "Double Click".into(),
            Self::FireButton => "Fire Button".into(),
            Self::ScrollUp => "Scroll Up".into(),
            Self::ScrollDown => "Scroll Down".into(),
            Self::DpiCycle => "DPI Cycle".into(),
            Self::DpiUp => "DPI +".into(),
            Self::DpiDown => "DPI -".into(),
            Self::EasyAim => "Easy Aim".into(),
            Self::KeyboardShortcut { modifiers, key } => {
                let mut parts = Vec::new();
                if modifiers & 0x01 != 0 { parts.push("Ctrl"); }
                if modifiers & 0x02 != 0 { parts.push("Shift"); }
                if modifiers & 0x04 != 0 { parts.push("Alt"); }
                if modifiers & 0x08 != 0 { parts.push("Win"); }
                parts.push(hid_key_name(*key));
                parts.join("+")
            }
            Self::Macro { slot } => format!("Macro (Slot {})", slot),
            Self::MediaPlayer => "Media Player".into(),
            Self::NextTrack => "Next Track".into(),
            Self::PlayPause => "Play / Pause".into(),
            Self::Mute => "Mute".into(),
            Self::VolumeUp => "Volume +".into(),
            Self::MyComputer => "My Computer".into(),
            Self::BrowserHome => "Browser Home".into(),
            Self::Raw(a, b, c) => format!("0x{:02X} {:02X} {:02X}", a, b, c),
        }
    }
}

pub fn hid_key_name(code: u8) -> &'static str {
    match code {
        0x04 => "A", 0x05 => "B", 0x06 => "C", 0x07 => "D", 0x08 => "E",
        0x09 => "F", 0x0A => "G", 0x0B => "H", 0x0C => "I", 0x0D => "J",
        0x0E => "K", 0x0F => "L", 0x10 => "M", 0x11 => "N", 0x12 => "O",
        0x13 => "P", 0x14 => "Q", 0x15 => "R", 0x16 => "S", 0x17 => "T",
        0x18 => "U", 0x19 => "V", 0x1A => "W", 0x1B => "X", 0x1C => "Y",
        0x1D => "Z", 0x1E => "1", 0x1F => "2", 0x20 => "3", 0x21 => "4",
        0x22 => "5", 0x23 => "6", 0x24 => "7", 0x25 => "8", 0x26 => "9",
        0x27 => "0", 0x28 => "Enter", 0x29 => "Esc", 0x2A => "Backspace",
        0x2B => "Tab", 0x2C => "Space", 0x3A => "F1", 0x3B => "F2",
        0x3C => "F3", 0x3D => "F4", 0x3E => "F5", 0x3F => "F6",
        0x40 => "F7", 0x41 => "F8", 0x42 => "F9", 0x43 => "F10",
        0x44 => "F11", 0x45 => "F12",
        _ => "Key",
    }
}

/// The 5 physical buttons on Attack Shark X3
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ButtonConfig {
    pub button1_left: ButtonAction,
    pub button2_right: ButtonAction,
    pub button3_middle: ButtonAction,
    pub button4_forward: ButtonAction,
    pub button5_backward: ButtonAction,
}

impl Default for ButtonConfig {
    fn default() -> Self {
        Self {
            button1_left: ButtonAction::LeftClick,
            button2_right: ButtonAction::RightClick,
            button3_middle: ButtonAction::MiddleClick,
            button4_forward: ButtonAction::Forward,
            button5_backward: ButtonAction::Backward,
        }
    }
}

/// Build the factory 18-slot table with physical button assignments
pub fn build_buttons_report(config: &ButtonConfig) -> Result<[u8; LEN_REPORT_BUTTONS]> {
    let mut slots = [[0u8; 3]; NUM_SLOTS];

    // Factory defaults for unassigned / reserved slots
    slots[3] = [0x0D, 0, 0]; // DPI Cycle
    slots[4] = [0x3C, 0, 0];
    slots[5] = [0x0F, 0, 0]; // DPI -
    slots[8] = [0x3C, 0, 0];
    for s in &mut slots[9..16] {
        *s = [0x01, 0, 0];
    }
    slots[16] = [0x0A, 0, 0]; // Wheel down
    slots[17] = [0x09, 0, 0]; // Wheel up

    // Wire physical buttons
    slots[BTN_LEFT_SLOT] = config.button1_left.to_bytes();
    slots[BTN_RIGHT_SLOT] = config.button2_right.to_bytes();
    slots[BTN_MIDDLE_SLOT] = config.button3_middle.to_bytes();
    slots[BTN_FORWARD_SLOT] = config.button4_forward.to_bytes();
    slots[BTN_BACKWARD_SLOT] = config.button5_backward.to_bytes();

    // Check safety rule: At least one button must be Left Click
    let has_left = slots.iter().any(|s| s[0] == 0x02);
    if !has_left {
        bail!("At least one button must be assigned to Left Click!");
    }

    let mut buf = [0u8; LEN_REPORT_BUTTONS];
    buf[0] = REPORT_BUTTONS;
    buf[1] = LEN_REPORT_BUTTONS as u8;
    buf[2] = PROFILE_INDEX;

    for i in 0..NUM_SLOTS {
        let offset = 3 + i * 3;
        buf[offset] = slots[i][0];
        buf[offset + 1] = slots[i][1];
        buf[offset + 2] = slots[i][2];
    }

    apply_checksum(&mut buf, CHECK_OFFSET_BUTTONS);

    Ok(buf)
}
