//! Attack Shark X3 HID Protocol Module.

pub mod battery;
pub mod buttons;
pub mod checksum;
pub mod constants;
pub mod macro_upload;
pub mod polling;
pub mod power;
pub mod sensor;

pub use battery::BatteryStatus;
pub use buttons::{ButtonConfig, build_buttons_report};
pub use constants::*;
pub use macro_upload::DeviceMacro;
pub use polling::{build_commit_report, build_polling_report};
pub use power::{PowerConfig, build_power_report};
pub use sensor::{SensorConfig, build_sensor_report};

use anyhow::Result;

/// A full set of packets ready to be transmitted to the mouse in the documented sequence
pub struct ApplyBurst {
    pub commit: [u8; LEN_REPORT_COMMIT],
    pub sensor: [u8; LEN_REPORT_SENSOR],
    pub power: [u8; LEN_REPORT_POWER],
    pub polling: [u8; LEN_REPORT_POLLING],
    pub buttons: [u8; LEN_REPORT_BUTTONS],
}

impl ApplyBurst {
    pub fn build(
        sensor_cfg: &SensorConfig,
        power_cfg: &PowerConfig,
        polling_rate: PollingRate,
        button_cfg: &ButtonConfig,
    ) -> Result<Self> {
        Ok(Self {
            commit: build_commit_report(),
            sensor: build_sensor_report(sensor_cfg)?,
            power: build_power_report(power_cfg)?,
            polling: build_polling_report(polling_rate),
            buttons: build_buttons_report(button_cfg)?,
        })
    }
}
