//! Macro manager modal window.

use eframe::egui::{self, Color32, Vec2};
use crate::device::DeviceManager;
use crate::gui::theme::*;
use crate::macros::{MacroEvent, MacroItem};

pub struct MacroDialog {
    pub selected_macro_idx: usize,
    pub status_msg: Option<String>,
}

impl Default for MacroDialog {
    fn default() -> Self {
        Self {
            selected_macro_idx: 0,
            status_msg: None,
        }
    }
}

impl MacroDialog {
    pub fn show(
        &mut self,
        ctx: &egui::Context,
        is_open: &mut bool,
        macros: &mut Vec<MacroItem>,
        device_mgr: &mut DeviceManager,
    ) {
        if !*is_open {
            return;
        }

        let mut open = *is_open;
        egui::Window::new("Macro Manager — Attack Shark X3")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .default_size(Vec2::new(480.0, 360.0))
            .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Left list of macros
                    ui.vertical(|ui| {
                        ui.set_width(140.0);
                        ui.label(egui::RichText::new("Macros:").strong());
                        egui::ScrollArea::vertical().max_height(240.0).show(ui, |ui| {
                            for (idx, m) in macros.iter().enumerate() {
                                let is_selected = idx == self.selected_macro_idx;
                                if ui.selectable_label(is_selected, &m.name).clicked() {
                                    self.selected_macro_idx = idx;
                                }
                            }
                        });

                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            if ui.button("+ Add").clicked() {
                                let new_m = MacroItem {
                                    name: format!("Macro {}", macros.len() + 1),
                                    ..Default::default()
                                };
                                macros.push(new_m);
                                self.selected_macro_idx = macros.len() - 1;
                            }
                            if ui.button("Delete").clicked() && macros.len() > 1 {
                                macros.remove(self.selected_macro_idx);
                                if self.selected_macro_idx >= macros.len() {
                                    self.selected_macro_idx = macros.len().saturating_sub(1);
                                }
                            }
                        });
                    });

                    ui.separator();

                    // Right side: Events list and actions
                    ui.vertical(|ui| {
                        if let Some(current) = macros.get_mut(self.selected_macro_idx) {
                            ui.horizontal(|ui| {
                                ui.label("Name:");
                                ui.text_edit_singleline(&mut current.name);
                            });

                            ui.add_space(6.0);
                            ui.label(egui::RichText::new("Events Sequence:").strong());

                            egui::ScrollArea::vertical().max_height(160.0).show(ui, |ui| {
                                egui::Grid::new("macro_events_grid")
                                    .striped(true)
                                    .show(ui, |ui| {
                                        ui.label(egui::RichText::new("Action").strong());
                                        ui.label(egui::RichText::new("Key").strong());
                                        ui.label(egui::RichText::new("").strong());
                                        ui.end_row();

                                        let mut to_remove = None;
                                        for (ev_idx, ev) in current.events.iter_mut().enumerate() {
                                            ui.label(if ev.is_release { "Release" } else { "Press" });
                                            ui.label(crate::protocol::buttons::hid_key_name(ev.hid_usage));
                                            if ui.small_button("×").clicked() {
                                                to_remove = Some(ev_idx);
                                            }
                                            ui.end_row();
                                        }

                                        if let Some(rem_idx) = to_remove {
                                            current.events.remove(rem_idx);
                                        }
                                    });
                            });

                            ui.add_space(8.0);
                            ui.horizontal(|ui| {
                                if ui.button("+ Add Keystroke (C)").clicked() {
                                    current.events.push(MacroEvent::press(0x06));
                                    current.events.push(MacroEvent::release(0x06));
                                }
                                if ui.button("Clear All").clicked() {
                                    current.events.clear();
                                }
                            });

                            ui.add_space(10.0);
                            ui.separator();
                            ui.add_space(6.0);

                            // Upload to mouse hardware
                            ui.horizontal(|ui| {
                                if ui
                                    .add_sized(
                                        [180.0, 26.0],
                                        egui::Button::new(
                                            egui::RichText::new("Upload to Mouse Slot 2")
                                                .color(Color32::WHITE)
                                                .strong(),
                                        )
                                        .fill(COLOR_ACCENT_RED),
                                    )
                                    .clicked()
                                {
                                    let dev_macro = current.to_device_macro();
                                    match device_mgr.upload_macro(&dev_macro) {
                                        Ok(_) => {
                                            self.status_msg = Some("Macro uploaded successfully to device!".into());
                                        }
                                        Err(e) => {
                                            self.status_msg = Some(format!("Error: {}", e));
                                        }
                                    }
                                }

                                if let Some(msg) = &self.status_msg {
                                    ui.label(egui::RichText::new(msg).color(COLOR_TEXT_SECONDARY).size(11.0));
                                }
                            });
                        }
                    });
                });
            });

        *is_open = open;
    }
}
