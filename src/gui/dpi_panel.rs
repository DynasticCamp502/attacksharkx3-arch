//! Right column: Advanced DPI stages configuration panel with custom values,
//! stage adding/removing, enable/disable toggles, RGB color picker, quick presets,
//! and dedicated custom DPI input field.

use eframe::egui::{self, Color32, Stroke, Vec2};
use crate::dpi::clamp_dpi;
use crate::gui::theme::*;
use crate::protocol::constants::DPI_SLOTS;
use crate::protocol::sensor::SensorConfig;

pub struct DpiPanel {
    pub pending_apply: bool,
    /// Persistent text edit buffers for each DPI stage
    pub text_buffers: Vec<String>,
    /// Buffer for dedicated custom DPI input
    pub custom_input: String,
}

impl Default for DpiPanel {
    fn default() -> Self {
        Self {
            pending_apply: false,
            text_buffers: Vec::new(),
            custom_input: String::new(),
        }
    }
}

impl DpiPanel {
    fn sync_buffers(&mut self, sensor: &SensorConfig) {
        if self.text_buffers.len() != sensor.dpi_stages.len() {
            self.text_buffers = sensor
                .dpi_stages
                .iter()
                .map(|&dpi| format!("{}", dpi))
                .collect();
        }

        let active_idx = sensor.active_stage.min(sensor.dpi_stages.len().saturating_sub(1));
        if self.custom_input.is_empty() {
            if let Some(&dpi) = sensor.dpi_stages.get(active_idx) {
                self.custom_input = format!("{}", dpi);
            }
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, sensor: &mut SensorConfig) {
        self.sync_buffers(sensor);

        section_header(ui, "DPI Settings");

        egui::Frame::none()
            .fill(COLOR_PANEL_BG)
            .inner_margin(10.0)
            .rounding(6.0)
            .stroke(Stroke::new(1.0_f32, COLOR_BORDER))
            .show(ui, |ui| {
                let active_idx = sensor.active_stage.min(sensor.dpi_stages.len().saturating_sub(1));

                // Top controls bar: Add/Remove Stage & Active Stage summary
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(format!(
                            "Stages: {} / {} (Active: Stage {})",
                            sensor.dpi_stages.len(),
                            DPI_SLOTS,
                            active_idx + 1
                        ))
                        .color(COLOR_TEXT_SECONDARY)
                        .size(12.0),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Add stage button
                        if sensor.dpi_stages.len() < DPI_SLOTS {
                            if ui
                                .button("+ Add Stage")
                                .on_hover_text("Add a new custom DPI stage (up to 8)")
                                .clicked()
                            {
                                let last_dpi = sensor.dpi_stages.last().copied().unwrap_or(1600);
                                let new_dpi = clamp_dpi((last_dpi + 800).min(26000));
                                sensor.dpi_stages.push(new_dpi);
                                self.text_buffers.push(format!("{}", new_dpi));

                                let default_colors = [
                                    [255, 0, 0], [0, 255, 0], [0, 0, 255], [255, 255, 0],
                                    [0, 255, 255], [255, 0, 255], [255, 64, 0], [255, 255, 255],
                                ];
                                let idx = sensor.dpi_stages.len() - 1;
                                if sensor.colors.len() <= idx {
                                    sensor.colors.push(default_colors[idx % default_colors.len()]);
                                }

                                let mut mask = sensor.enabled_mask.unwrap_or(0);
                                mask |= 1 << idx;
                                sensor.enabled_mask = Some(mask);
                                self.pending_apply = true;
                            }
                        }

                        // Remove stage button
                        if sensor.dpi_stages.len() > 1 {
                            if ui
                                .button("- Remove Stage")
                                .on_hover_text("Remove the last DPI stage")
                                .clicked()
                            {
                                let idx = sensor.dpi_stages.len() - 1;
                                sensor.dpi_stages.pop();
                                self.text_buffers.pop();
                                if sensor.colors.len() > sensor.dpi_stages.len() {
                                    sensor.colors.pop();
                                }
                                if sensor.active_stage >= sensor.dpi_stages.len() {
                                    sensor.active_stage = sensor.dpi_stages.len() - 1;
                                }
                                if let Some(ref mut mask) = sensor.enabled_mask {
                                    *mask &= !(1 << idx);
                                }
                                self.pending_apply = true;
                            }
                        }
                    });
                });

                ui.add_space(8.0);

                // Dedicated Custom DPI Input for Active Stage
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Custom DPI:")
                            .color(COLOR_TEXT_PRIMARY)
                            .strong()
                            .size(12.0),
                    );

                    let edit_resp = ui.add_sized(
                        [80.0, 24.0],
                        egui::TextEdit::singleline(&mut self.custom_input)
                            .font(egui::FontId::monospace(12.0))
                            .desired_width(80.0)
                            .hint_text("50-26000"),
                    );

                    let is_custom_focused = edit_resp.has_focus();
                    let enter_pressed = is_custom_focused && ui.input(|i| i.key_pressed(egui::Key::Enter));
                    let set_clicked = ui
                        .add_sized(
                            [46.0, 24.0],
                            egui::Button::new(
                                egui::RichText::new("Set")
                                    .color(Color32::WHITE)
                                    .strong(),
                            )
                            .fill(COLOR_ACCENT_RED),
                        )
                        .on_hover_text("Apply custom DPI to active stage")
                        .clicked();

                    if set_clicked || enter_pressed || edit_resp.lost_focus() {
                        if let Ok(val) = self.custom_input.trim().parse::<u32>() {
                            let clamped = clamp_dpi(val);
                            sensor.dpi_stages[active_idx] = clamped;
                            self.custom_input = format!("{}", clamped);
                            if let Some(buf) = self.text_buffers.get_mut(active_idx) {
                                *buf = format!("{}", clamped);
                            }
                            self.pending_apply = true;
                        } else if !is_custom_focused {
                            self.custom_input = format!("{}", sensor.dpi_stages[active_idx]);
                        }
                    } else if !is_custom_focused {
                        // Keep synced if user is not actively editing
                        self.custom_input = format!("{}", sensor.dpi_stages[active_idx]);
                    }

                    // Quick adjustments
                    if ui.small_button("-50").on_hover_text("Decrease 50 DPI").clicked() {
                        let cur = sensor.dpi_stages[active_idx];
                        let new_dpi = clamp_dpi(cur.saturating_sub(50));
                        sensor.dpi_stages[active_idx] = new_dpi;
                        self.custom_input = format!("{}", new_dpi);
                        if let Some(buf) = self.text_buffers.get_mut(active_idx) {
                            *buf = format!("{}", new_dpi);
                        }
                        self.pending_apply = true;
                    }
                    if ui.small_button("+50").on_hover_text("Increase 50 DPI").clicked() {
                        let cur = sensor.dpi_stages[active_idx];
                        let new_dpi = clamp_dpi((cur + 50).min(26000));
                        sensor.dpi_stages[active_idx] = new_dpi;
                        self.custom_input = format!("{}", new_dpi);
                        if let Some(buf) = self.text_buffers.get_mut(active_idx) {
                            *buf = format!("{}", new_dpi);
                        }
                        self.pending_apply = true;
                    }
                });

                ui.add_space(6.0);

                // Quick presets row
                ui.horizontal_wrapped(|ui| {
                    ui.label(egui::RichText::new("Presets:").size(11.0).color(COLOR_TEXT_MUTED));
                    for &preset in &[400, 800, 1200, 1600, 2400, 3200, 6400, 12000, 26000] {
                        let is_current = sensor.dpi_stages[active_idx] == preset;
                        let text = egui::RichText::new(format!("{}", preset))
                            .size(10.5)
                            .color(if is_current { Color32::WHITE } else { COLOR_TEXT_SECONDARY });

                        if ui
                            .selectable_label(is_current, text)
                            .on_hover_text(format!("Set Active Stage {} to {} DPI", active_idx + 1, preset))
                            .clicked()
                        {
                            sensor.dpi_stages[active_idx] = preset;
                            self.custom_input = format!("{}", preset);
                            if let Some(buf) = self.text_buffers.get_mut(active_idx) {
                                *buf = format!("{}", preset);
                            }
                            self.pending_apply = true;
                        }
                    }
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                // Responsive Stage Cards Grid
                let stages_count = sensor.dpi_stages.len();
                let avail_w = ui.available_width();
                let col_width = ((avail_w - (stages_count as f32 * 8.0)) / stages_count as f32)
                    .clamp(48.0, 85.0);

                let mut mask = sensor.enabled_mask.unwrap_or_else(|| ((1 << stages_count) - 1) as u8);

                ui.horizontal(|ui| {
                    for i in 0..stages_count {
                        let is_active = sensor.active_stage == i;
                        let is_enabled = (mask & (1 << i)) != 0;

                        ui.vertical(|ui| {
                            ui.set_width(col_width);

                            // Header with Stage Number & Radio Selection
                            ui.horizontal(|ui| {
                                if ui
                                    .radio(is_active, format!("{}", i + 1))
                                    .on_hover_text(format!("Select Stage {} as active DPI", i + 1))
                                    .clicked()
                                {
                                    sensor.active_stage = i;
                                    self.custom_input = format!("{}", sensor.dpi_stages[i]);
                                    self.pending_apply = true;
                                }
                            });

                            // Enabled checkbox
                            let mut checked = is_enabled;
                            if ui
                                .checkbox(&mut checked, "On")
                                .on_hover_text("Enable or disable this DPI stage")
                                .changed()
                            {
                                if checked {
                                    mask |= 1 << i;
                                } else if mask.count_ones() > 1 {
                                    mask &= !(1 << i);
                                }
                                sensor.enabled_mask = Some(mask);
                                self.pending_apply = true;
                            }

                            ui.add_space(2.0);

                            // Color Picker for stage indicator
                            if let Some(rgb) = sensor.colors.get_mut(i) {
                                ui.horizontal(|ui| {
                                    if ui.color_edit_button_srgb(rgb).changed() {
                                        self.pending_apply = true;
                                    }
                                });
                            }

                            ui.add_space(4.0);

                            // Max indicator
                            ui.label(egui::RichText::new("26k").size(9.0).color(COLOR_TEXT_MUTED));

                            // Vertical Slider
                            let current_dpi = sensor.dpi_stages[i];
                            let mut slider_val = current_dpi as f64;
                            let slider = egui::Slider::new(&mut slider_val, 50.0..=26000.0)
                                .step_by(50.0)
                                .vertical()
                                .show_value(false);

                            let slider_resp = ui.add_sized([col_width - 10.0, 110.0], slider);
                            if slider_resp.changed() {
                                let new_dpi = clamp_dpi(slider_val as u32);
                                sensor.dpi_stages[i] = new_dpi;
                                if i == active_idx {
                                    self.custom_input = format!("{}", new_dpi);
                                }
                                if let Some(buf) = self.text_buffers.get_mut(i) {
                                    *buf = format!("{}", new_dpi);
                                }
                            }
                            if slider_resp.drag_stopped() {
                                self.pending_apply = true;
                            }

                            // Min indicator
                            ui.label(egui::RichText::new("50").size(9.0).color(COLOR_TEXT_MUTED));

                            ui.add_space(2.0);

                            // Fine-tuning buttons (-50 / +50)
                            ui.horizontal(|ui| {
                                if ui.small_button("-").on_hover_text("-50 DPI").clicked() {
                                    let new_dpi = clamp_dpi(sensor.dpi_stages[i].saturating_sub(50));
                                    sensor.dpi_stages[i] = new_dpi;
                                    if i == active_idx {
                                        self.custom_input = format!("{}", new_dpi);
                                    }
                                    if let Some(buf) = self.text_buffers.get_mut(i) {
                                        *buf = format!("{}", new_dpi);
                                    }
                                    self.pending_apply = true;
                                }
                                if ui.small_button("+").on_hover_text("+50 DPI").clicked() {
                                    let new_dpi = clamp_dpi((sensor.dpi_stages[i] + 50).min(26000));
                                    sensor.dpi_stages[i] = new_dpi;
                                    if i == active_idx {
                                        self.custom_input = format!("{}", new_dpi);
                                    }
                                    if let Some(buf) = self.text_buffers.get_mut(i) {
                                        *buf = format!("{}", new_dpi);
                                    }
                                    self.pending_apply = true;
                                }
                            });

                            ui.add_space(4.0);

                            // Editable DPI Text Box with high-contrast styling
                            if let Some(buf) = self.text_buffers.get_mut(i) {
                                let edit_box = egui::Frame::none()
                                    .fill(Color32::from_rgb(0x18, 0x18, 0x18))
                                    .stroke(Stroke::new(1.0_f32, if is_active { COLOR_ACCENT_RED } else { COLOR_BORDER }))
                                    .rounding(3.0)
                                    .inner_margin(Vec2::new(2.0, 1.0))
                                    .show(ui, |ui| {
                                        ui.add_sized(
                                            [col_width - 8.0, 20.0],
                                            egui::TextEdit::singleline(buf)
                                                .font(egui::FontId::monospace(10.5))
                                                .desired_width(col_width - 8.0)
                                                .vertical_align(egui::Align::Center),
                                        )
                                    });

                                let text_edit = edit_box.inner;
                                let is_focused = text_edit.has_focus();
                                let enter = is_focused && ui.input(|i| i.key_pressed(egui::Key::Enter));

                                if text_edit.lost_focus() || enter {
                                    if let Ok(parsed) = buf.trim().parse::<u32>() {
                                        let clamped = clamp_dpi(parsed);
                                        sensor.dpi_stages[i] = clamped;
                                        *buf = format!("{}", clamped);
                                        if i == active_idx {
                                            self.custom_input = format!("{}", clamped);
                                        }
                                        self.pending_apply = true;
                                    } else {
                                        *buf = format!("{}", sensor.dpi_stages[i]);
                                    }
                                } else if !is_focused {
                                    *buf = format!("{}", sensor.dpi_stages[i]);
                                }
                            }
                        });
                    }
                });

                ui.add_space(10.0);

                // Apply DPI Button
                if ui
                    .add_sized(
                        [ui.available_width(), 28.0],
                        egui::Button::new(
                            egui::RichText::new("Apply DPI Settings")
                                .color(Color32::WHITE)
                                .strong(),
                        )
                        .fill(COLOR_ACCENT_RED),
                    )
                    .clicked()
                {
                    self.pending_apply = true;
                }
            });
    }
}
