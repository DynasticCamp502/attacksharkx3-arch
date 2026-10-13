//! Button configuration domain model.

pub use crate::protocol::buttons::{ButtonAction, ButtonConfig};

/// List of standard available button actions for GUI selector
pub fn available_actions() -> &'static [ButtonAction] {
    &[
        ButtonAction::LeftClick,
        ButtonAction::RightClick,
        ButtonAction::MiddleClick,
        ButtonAction::Forward,
        ButtonAction::Backward,
        ButtonAction::DoubleClick,
        ButtonAction::FireButton,
        ButtonAction::ScrollUp,
        ButtonAction::ScrollDown,
        ButtonAction::DpiCycle,
        ButtonAction::DpiUp,
        ButtonAction::DpiDown,
        ButtonAction::EasyAim,
        ButtonAction::MediaPlayer,
        ButtonAction::PlayPause,
        ButtonAction::NextTrack,
        ButtonAction::Mute,
        ButtonAction::VolumeUp,
        ButtonAction::MyComputer,
        ButtonAction::BrowserHome,
        ButtonAction::ButtonOff,
    ]
}
