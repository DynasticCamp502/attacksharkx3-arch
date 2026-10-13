//! Profiles management and Battery display panel.

use eframe::egui::{self, Color32, ProgressBar, Rounding, Stroke, Vec2};
use crate::gui::theme::*;
use crate::profiles::ProfileManager;
use crate::protocol::BatteryStatus;

pub struct ProfilesPanel {
    pub editing_name: Option<String>,
}

impl Default for ProfilesPanel {
    fn default() -> Self {
        Self { editing_name: None }
    }
}

impl ProfilesPanel {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        profiles: &mut ProfileManager,
        battery: Option<BatteryStatus>,
        on_profile_changed: &mut bool,
    ) {
        // Section: Profiles
        section_header(ui, "Profile");

        egui::Frame::none()
            .fill(COLOR_PANEL_BG)
            .inner_margin(8.0)
            .rounding(4.0)
            .stroke(Stroke::new(1.0, COLOR_BORDER))
            .show(ui, |ui| {
                ui.set_height(90.0);
                egui::ScrollArea::vertical()
                    .max_height(80.0)
                    .show(ui, |ui| {
                        let active_idx = profiles.active_index;
                        for i in 0..profiles.profiles.len() {
                            let name = profiles.profiles[i].name.clone();
                            let is_active = i == active_idx;
                            let text = egui::RichText::new(&name)
                                .color(if is_active { COLOR_TEXT_PRIMARY } else { COLOR_TEXT_SECONDARY })
                                .strong();

                            if ui
                                .selectable_label(is_active, text)
                                .clicked()
                            {
                                if profiles.active_index != i {
                                    profiles.active_index = i;
                                    *on_profile_changed = true;
                                }
                            }
                        }
                    });

                // Profile action buttons bar (+, delete, rename, reset)
                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    if ui.button("+").on_hover_text("Add Profile").clicked() {
                        let count = profiles.profiles.len() + 1;
                        profiles.add_profile(format!("Profile {}", count));
                        *on_profile_changed = true;
                    }

                    if ui.button("🗑").on_hover_text("Delete Profile").clicked() {
                        if profiles.delete_profile(profiles.active_index) {
                            *on_profile_changed = true;
                        }
                    }

                    if ui.button("↺").on_hover_text("Reset to defaults").clicked() {
                        profiles.reset_active();
                        *on_profile_changed = true;
                    }
                });
            });

        ui.add_space(10.0);

        // Section: Battery
        section_header(ui, "Battery");
        egui::Frame::none()
            .fill(COLOR_PANEL_BG)
            .inner_margin(8.0)
            .rounding(4.0)
            .stroke(Stroke::new(1.0, COLOR_BORDER))
            .show(ui, |ui| {
                if let Some(bat) = battery {
                    let fraction = (bat.percentage as f32) / 100.0;
                    ui.add(
                        ProgressBar::new(fraction)
                            .desired_height(14.0)
                            .desired_width(ui.available_width())
                            .rounding(Rounding::same(3.0))
                            .fill(COLOR_BATTERY_GREEN)
                            .text(format!("{}%", bat.percentage)),
                    );
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(format!(
                            "{:.2} V {}",
                            bat.voltage,
                            if bat.is_charging { "(Charging)" } else { "" }
                        ))
                        .color(COLOR_TEXT_SECONDARY)
                        .size(11.0),
                    );
                } else {
                    ui.label(
                        egui::RichText::new("Battery level unknown (wake mouse)")
                            .color(COLOR_TEXT_MUTED)
                            .size(11.5),
                    );
                }
            });
    }
}
