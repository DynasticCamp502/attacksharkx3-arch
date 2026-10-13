//! Performance and sensor attributes panel.

use eframe::egui::{self, Stroke};
use crate::gui::theme::*;
use crate::protocol::{
    constants::{LiftOffDistance, PollingRate},
    power::PowerConfig,
    sensor::SensorConfig,
};

pub struct PerfPanel;

impl Default for PerfPanel {
    fn default() -> Self {
        Self
    }
}

impl PerfPanel {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        sensor: &mut SensorConfig,
        power: &mut PowerConfig,
        polling_rate: &mut PollingRate,
        pending_apply: &mut bool,
    ) {
        // Section: Polling Rate
        section_header(ui, "Polling Rate");
        egui::Frame::none()
            .fill(COLOR_PANEL_BG)
            .inner_margin(8.0)
            .rounding(4.0)
            .stroke(Stroke::new(1.0, COLOR_BORDER))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    for &rate in &[
                        PollingRate::Hz125,
                        PollingRate::Hz250,
                        PollingRate::Hz500,
                        PollingRate::Hz1000,
                    ] {
                        let is_selected = *polling_rate == rate;
                        let text = egui::RichText::new(format!("{}Hz", rate as u16))
                            .color(if is_selected { COLOR_TEXT_PRIMARY } else { COLOR_TEXT_SECONDARY })
                            .size(11.5);
                        if ui
                            .selectable_label(is_selected, text)
                            .on_hover_text(rate.label())
                            .clicked()
                        {
                            *polling_rate = rate;
                            *pending_apply = true;
                        }
                    }
                });
            });

        ui.add_space(8.0);

        // Section: Sensor & Performance
        section_header(ui, "Performance & Sensor");
        egui::Frame::none()
            .fill(COLOR_PANEL_BG)
            .inner_margin(8.0)
            .rounding(4.0)
            .stroke(Stroke::new(1.0, COLOR_BORDER))
            .show(ui, |ui| {
                // Lift-off distance (1mm / 2mm)
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Lift Off Distance:").size(12.0));
                    if ui.radio(sensor.lod == LiftOffDistance::OneMm, "1 mm").clicked() {
                        sensor.lod = LiftOffDistance::OneMm;
                        *pending_apply = true;
                    }
                    if ui.radio(sensor.lod == LiftOffDistance::TwoMm, "2 mm").clicked() {
                        sensor.lod = LiftOffDistance::TwoMm;
                        *pending_apply = true;
                    }
                });

                ui.add_space(4.0);

                // Key Debounce Time (2 .. 50 ms)
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Debounce:").size(12.0));
                    let mut debounce_val = power.key_debounce_ms as f64;
                    if ui
                        .add(egui::Slider::new(&mut debounce_val, 2.0..=50.0).step_by(2.0).suffix(" ms"))
                        .changed()
                    {
                        power.key_debounce_ms = debounce_val as u8;
                        *pending_apply = true;
                    }
                });

                ui.add_space(4.0);

                // Sleep Time (0.5 .. 30 min)
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Sleep Time:").size(12.0));
                    let mut sleep_val = power.sleep_minutes as f64;
                    if ui
                        .add(egui::Slider::new(&mut sleep_val, 0.5..=30.0).step_by(0.5).suffix(" min"))
                        .changed()
                    {
                        power.sleep_minutes = sleep_val as f32;
                        *pending_apply = true;
                    }
                });

                ui.add_space(4.0);

                // Deep Sleep Time (1 .. 60 min)
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Deep Sleep:").size(12.0));
                    let mut deep_val = power.deep_sleep_minutes as f64;
                    if ui
                        .add(egui::Slider::new(&mut deep_val, 1.0..=60.0).step_by(1.0).suffix(" min"))
                        .changed()
                    {
                        power.deep_sleep_minutes = deep_val as u8;
                        *pending_apply = true;
                    }
                });

                ui.add_space(6.0);
                ui.separator();
                ui.add_space(4.0);

                // Toggles: Ripple, Angle Snap, Motion Sync
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut sensor.ripple_control, "Ripple Control").changed() {
                        *pending_apply = true;
                    }
                });
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut sensor.angle_snap, "Angle Snap").changed() {
                        *pending_apply = true;
                    }
                });
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut sensor.motion_sync, "Motion Sync").changed() {
                        *pending_apply = true;
                    }
                });
            });
    }
}
