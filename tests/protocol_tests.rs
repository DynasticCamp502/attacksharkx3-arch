//! Integration tests for Attack Shark X3 HID Protocol.

use attack_shark_x3::protocol::{
    battery::BatteryStatus,
    buttons::{build_buttons_report, ButtonAction, ButtonConfig},
    checksum::{verify_checksum, verify_macro_checksum},
    constants::*,
    macro_upload::{DeviceMacro, MacroEvent},
    polling::{build_commit_report, build_polling_report},
    power::{build_power_report, PowerConfig},
    sensor::{build_sensor_report, dpi_to_raw, raw_to_dpi, SensorConfig},
    ApplyBurst,
};

#[test]
fn test_dpi_conversion() {
    assert_eq!(dpi_to_raw(800).unwrap(), 15);
    assert_eq!(dpi_to_raw(1600).unwrap(), 31);
    assert_eq!(dpi_to_raw(2400).unwrap(), 47);
    assert_eq!(dpi_to_raw(3200).unwrap(), 63);
    assert_eq!(dpi_to_raw(6400).unwrap(), 127);
    assert_eq!(dpi_to_raw(26000).unwrap(), 519); // 0x0207 -> low 0x07, high 0x02

    // Roundtrip tests
    for dpi in (50..=26000).step_by(50) {
        let raw = dpi_to_raw(dpi).unwrap();
        assert_eq!(raw_to_dpi(raw), dpi);
    }
}

#[test]
fn test_sensor_report_building_and_checksum() {
    let config = SensorConfig::default();
    let report = build_sensor_report(&config).unwrap();

    assert_eq!(report.len(), LEN_REPORT_SENSOR);
    assert_eq!(report[0], REPORT_SENSOR);
    assert_eq!(report[1], 56);
    assert_eq!(report[2], 0x01); // Profile index

    // Verify low bytes: 800 (15 = 0x0f), 1600 (31 = 0x1f), 2400 (47 = 0x2f), 3200 (63 = 0x3f), 6400 (127 = 0x7f), 26000 (0x07)
    assert_eq!(report[8], 0x0F);
    assert_eq!(report[9], 0x1F);
    assert_eq!(report[10], 0x2F);
    assert_eq!(report[11], 0x3F);
    assert_eq!(report[12], 0x7F);
    assert_eq!(report[13], 0x07);

    // Verify high bytes: 26000 is 519 = 0x0207 -> high byte is 0x02
    assert_eq!(report[16], 0x00);
    assert_eq!(report[21], 0x02);

    // Verify active stage is 1-based on the wire (active_stage 1 => 2 on wire)
    assert_eq!(report[24], 2);

    // Verify constant at byte 49
    assert_eq!(report[49], 0x01);

    // Verify checksum
    assert!(verify_checksum(&report, CHECK_OFFSET_SENSOR));
}

#[test]
fn test_power_report_building_and_checksum() {
    let config = PowerConfig {
        sleep_minutes: 5.0,
        deep_sleep_minutes: 30,
        key_debounce_ms: 4,
    };
    let report = build_power_report(&config).unwrap();

    assert_eq!(report.len(), LEN_REPORT_POWER);
    assert_eq!(report[0], REPORT_POWER);
    assert_eq!(report[1], 15);
    assert_eq!(report[2], 0x01);

    // Deep sleep 30 (0x1E) split: (30 & 0xF0) | 0x03 = 0x13; ((30 & 0x0F) << 4) | 0x08 = 0xE8
    assert_eq!(report[4], 0x13);
    assert_eq!(report[5], 0xE8);

    // Sleep 5.0 min * 2 = 10 (0x0A)
    assert_eq!(report[9], 10);

    // Debounce 4 ms / 2 = 2
    assert_eq!(report[10], 2);

    // Verify checksum
    assert!(verify_checksum(&report, CHECK_OFFSET_POWER));
}

#[test]
fn test_polling_rate_reports() {
    let r1000 = build_polling_report(PollingRate::Hz1000);
    assert_eq!(r1000[0], REPORT_POLLING);
    assert_eq!(r1000[1], 9);
    assert_eq!(r1000[3], 0x01);
    assert_eq!(r1000[4], 0xFE);

    let r500 = build_polling_report(PollingRate::Hz500);
    assert_eq!(r500[3], 0x02);
    assert_eq!(r500[4], 0xFD);

    let r250 = build_polling_report(PollingRate::Hz250);
    assert_eq!(r250[3], 0x04);
    assert_eq!(r250[4], 0xFB);

    let r125 = build_polling_report(PollingRate::Hz125);
    assert_eq!(r125[3], 0x08);
    assert_eq!(r125[4], 0xF7);
}

#[test]
fn test_commit_report() {
    let commit = build_commit_report();
    assert_eq!(commit.len(), 10);
    assert_eq!(commit[0], REPORT_COMMIT);
    assert_eq!(commit[1], 10);
    assert_eq!(&commit[2..6], &[0x01, 0xFE, 0x01, 0xFE]);
}

#[test]
fn test_buttons_report_building_and_checksum() {
    let mut config = ButtonConfig::default();
    config.button4_forward = ButtonAction::KeyboardShortcut {
        modifiers: 0x01, // Ctrl
        key: 0x1B,       // 'X'
    };

    let report = build_buttons_report(&config).unwrap();
    assert_eq!(report.len(), LEN_REPORT_BUTTONS);
    assert_eq!(report[0], REPORT_BUTTONS);
    assert_eq!(report[1], 59);

    // Check Button 1 (Left click = 0x02, slot 0 at offset 3)
    assert_eq!(report[3], 0x02);

    // Check Button 4 (slot 6 at offset 3 + 6*3 = 21) -> Shortcut Ctrl+X
    assert_eq!(report[21], 0x11);
    assert_eq!(report[22], 0x01);
    assert_eq!(report[23], 0x1B);

    assert!(verify_checksum(&report, CHECK_OFFSET_BUTTONS));
}

#[test]
fn test_macro_upload_block_and_chunks() {
    let mut m = DeviceMacro::default();
    m.slot = 2;
    // 7 taps of 'C' key (usage 0x06)
    for _ in 0..7 {
        m.events.push(MacroEvent::press(0x06));
        m.events.push(MacroEvent::release(0x06));
    }

    let block = m.build_block().unwrap();
    assert_eq!(block.len(), 128);
    assert_eq!(block[0], 2); // Slot 2
    assert_eq!(block[4], 0x01);
    assert_eq!(block[25], 14); // 14 events

    // First event: press 'C' -> [0x01, 0x06]
    assert_eq!(block[26], 0x01);
    assert_eq!(block[27], 0x06);
    // Second event: release 'C' -> [0x81, 0x06]
    assert_eq!(block[28], 0x81);
    assert_eq!(block[29], 0x06);

    // Checksum verified: 2 (slot) + 0x01 + 0x0E + 7 * (0x01 + 0x06 + 0x81 + 0x06) = 0x03F3
    assert_eq!(block[126], 0x03);
    assert_eq!(block[127], 0xF3);
    assert!(verify_macro_checksum(&block));

    let (c0, c1, c2) = m.build_chunks().unwrap();
    assert_eq!(c0.len(), 64);
    assert_eq!(c1.len(), 64);
    assert_eq!(c2.len(), 12);
    assert_eq!(c0[0], REPORT_MACRO);
    assert_eq!(c0[1], 0x40);
    assert_eq!(c0[3], 0x00); // chunk index 0
    assert_eq!(c1[3], 0x01); // chunk index 1
    assert_eq!(c2[3], 0x02); // chunk index 2
}

#[test]
fn test_battery_status_parsing() {
    let report = [0x03, 0x10, 0x40, 0x01, 0x09];
    let status = BatteryStatus::parse(&report).unwrap();
    assert_eq!(status.raw_byte, 0x40);
    assert_eq!(status.voltage, 4.0);
    assert!(status.percentage >= 70 && status.percentage <= 85);
}

#[test]
fn test_apply_burst_assembly() {
    let sensor = SensorConfig::default();
    let power = PowerConfig::default();
    let rate = PollingRate::Hz1000;
    let buttons = ButtonConfig::default();

    let burst = ApplyBurst::build(&sensor, &power, rate, &buttons).unwrap();
    assert_eq!(burst.commit[0], REPORT_COMMIT);
    assert_eq!(burst.sensor[0], REPORT_SENSOR);
    assert_eq!(burst.power[0], REPORT_POWER);
    assert_eq!(burst.polling[0], REPORT_POLLING);
    assert_eq!(burst.buttons[0], REPORT_BUTTONS);
}

#[test]
fn test_dpi_conversion_invalid_cases() {
    assert!(dpi_to_raw(0).is_err());
    assert!(dpi_to_raw(49).is_err());
    assert!(dpi_to_raw(26001).is_err());
    assert!(dpi_to_raw(26050).is_err());
    assert!(dpi_to_raw(75).is_err()); // Not a multiple of 50
}

#[test]
fn test_clamp_dpi_boundaries() {
    use attack_shark_x3::dpi::clamp_dpi;

    assert_eq!(clamp_dpi(0), 50);
    assert_eq!(clamp_dpi(25), 50);
    assert_eq!(clamp_dpi(49), 50);
    assert_eq!(clamp_dpi(50), 50);
    assert_eq!(clamp_dpi(74), 50);
    assert_eq!(clamp_dpi(75), 100);
    assert_eq!(clamp_dpi(25980), 26000);
    assert_eq!(clamp_dpi(26000), 26000);
    assert_eq!(clamp_dpi(30000), 26000);
}

#[test]
fn test_sensor_report_empty_stages() {
    let mut config = SensorConfig::default();
    config.dpi_stages = vec![];
    let res = build_sensor_report(&config);
    assert!(res.is_err());
}

#[test]
fn test_power_report_invalid_inputs() {
    // sleep_minutes too low (raw value 0)
    let p_low = PowerConfig {
        sleep_minutes: 0.0,
        deep_sleep_minutes: 30,
        key_debounce_ms: 4,
    };
    assert!(build_power_report(&p_low).is_err());

    // sleep_minutes too high (raw value > 60)
    let p_high = PowerConfig {
        sleep_minutes: 35.0,
        deep_sleep_minutes: 30,
        key_debounce_ms: 4,
    };
    assert!(build_power_report(&p_high).is_err());

    // deep_sleep_minutes out of range (0 or >60)
    let p_deep_zero = PowerConfig {
        sleep_minutes: 5.0,
        deep_sleep_minutes: 0,
        key_debounce_ms: 4,
    };
    assert!(build_power_report(&p_deep_zero).is_err());

    let p_deep_high = PowerConfig {
        sleep_minutes: 5.0,
        deep_sleep_minutes: 61,
        key_debounce_ms: 4,
    };
    assert!(build_power_report(&p_deep_high).is_err());

    // debounce out of range
    let p_deb_zero = PowerConfig {
        sleep_minutes: 5.0,
        deep_sleep_minutes: 30,
        key_debounce_ms: 0,
    };
    assert!(build_power_report(&p_deb_zero).is_err());

    let p_deb_high = PowerConfig {
        sleep_minutes: 5.0,
        deep_sleep_minutes: 30,
        key_debounce_ms: 52,
    };
    assert!(build_power_report(&p_deb_high).is_err());
}

#[test]
fn test_buttons_report_missing_left_click_rejected() {
    let mut config = ButtonConfig::default();
    config.button1_left = ButtonAction::RightClick; // No LeftClick anywhere
    let res = build_buttons_report(&config);
    assert!(res.is_err());
}

#[test]
fn test_macro_invalid_slot_and_capacity_limits() {
    // Slot 0 rejected
    let mut m = DeviceMacro::default();
    m.slot = 0;
    assert!(m.build_block().is_err());

    // Exceeding capacity (> 50 events)
    let mut m2 = DeviceMacro::default();
    m2.slot = 2;
    for _ in 0..51 {
        m2.events.push(MacroEvent::press(0x04));
    }
    assert!(m2.build_block().is_err());
}
