//! Main eframe application coordinating GUI and device communication.

use eframe::egui::{self, Color32, Vec2};
use std::time::{Duration, Instant};
use tracing::{error, info};

use crate::config::AppConfig;
use crate::device::{ConnectionType, DeviceManager};
use crate::gui::*;
use crate::protocol::ApplyBurst;

pub struct AttackSharkApp {
    config: AppConfig,
    device: DeviceManager,
    buttons_panel: ButtonsPanel,
    profiles_panel: ProfilesPanel,
    mouse_view: MouseView,
    dpi_panel: DpiPanel,
    perf_panel: PerfPanel,
    macro_dialog: MacroDialog,

    last_poll_time: Instant,
    status_message: Option<(String, Instant)>,
}

impl AttackSharkApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        theme::apply_theme(&cc.egui_ctx);
        let config = AppConfig::load();
        let mut device = DeviceManager::new();
        device.refresh_connection();

        Self {
            config,
            device,
            buttons_panel: ButtonsPanel::default(),
            profiles_panel: ProfilesPanel::default(),
            mouse_view: MouseView::default(),
            dpi_panel: DpiPanel::default(),
            perf_panel: PerfPanel::default(),
            macro_dialog: MacroDialog::default(),
            last_poll_time: Instant::now(),
            status_message: Some(("Ready".into(), Instant::now())),
        }
    }

    fn set_status(&mut self, text: impl Into<String>) {
        self.status_message = Some((text.into(), Instant::now()));
    }

    fn apply_current_configuration(&mut self) {
        let profile = self.config.profiles.active_profile();
        match ApplyBurst::build(
            &profile.sensor,
            &profile.power,
            profile.polling_rate,
            &profile.buttons,
        ) {
            Ok(burst) => match self.device.send_burst(&burst) {
                Ok(_) => {
                    self.set_status("Settings applied successfully to mouse!");
                    let _ = self.config.save();
                }
                Err(e) => {
                    error!("Failed to apply settings: {}", e);
                    self.set_status(format!("Error: {}", e));
                }
            },
            Err(e) => {
                error!("Packet generation error: {}", e);
                self.set_status(format!("Config error: {}", e));
            }
        }
    }
}

impl eframe::App for AttackSharkApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Periodic background poll every 2 seconds
        if self.last_poll_time.elapsed() >= Duration::from_secs(2) {
            self.last_poll_time = Instant::now();
            self.device.refresh_connection();
            self.device.poll_battery();
            ctx.request_repaint_after(Duration::from_millis(500));
        }

        let mut trigger_apply = false;
        let mut profile_switched = false;

        // Top Navigation Bar
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("ATTACK SHARK X3")
                        .color(COLOR_TEXT_PRIMARY)
                        .strong()
                        .size(16.0),
                );
                ui.label(
                    egui::RichText::new("Linux Driver")
                        .color(COLOR_ACCENT_RED)
                        .size(13.0),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui
                        .add_sized(
                            [120.0, 26.0],
                            egui::Button::new(
                                egui::RichText::new("Apply All")
                                    .color(Color32::WHITE)
                                    .strong(),
                            )
                            .fill(COLOR_ACCENT_RED),
                        )
                        .clicked()
                    {
                        trigger_apply = true;
                    }

                    // Device connection badge
                    let (badge_color, badge_text) = match self.device.connection_type {
                        ConnectionType::Wireless24G => (COLOR_CONNECTED_GREEN, "● 2.4G"),
                        ConnectionType::WiredUsb => (COLOR_CONNECTED_GREEN, "● Wired"),
                        ConnectionType::PermissionDenied => (COLOR_ACCENT_RED, "● udev permission denied"),
                        ConnectionType::Disconnected => (COLOR_TEXT_MUTED, "○ Disconnected"),
                    };
                    ui.label(egui::RichText::new(badge_text).color(badge_color).size(12.0));
                });
            });
            ui.add_space(4.0);
        });

        // Bottom Status Bar
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if let Some((msg, _)) = &self.status_message {
                    ui.label(egui::RichText::new(msg).color(COLOR_TEXT_SECONDARY).size(11.5));
                }
                if self.device.connection_type == ConnectionType::PermissionDenied {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new("Run 'sudo cp packaging/udev/99-attack-shark-x3.rules /etc/udev/rules.d/'")
                                .color(COLOR_ACCENT_RED)
                                .size(11.0),
                        );
                    });
                }
            });
            ui.add_space(4.0);
        });

        // Main Central View with 3 columns (responsive & centered)
        egui::CentralPanel::default().show(ctx, |ui| {
            let total_width = ui.available_width();
            let content_width = total_width.min(1420.0);
            let margin_x = ((total_width - content_width) / 2.0).max(0.0);

            let col_gap = 14.0;
            let usable_w = (content_width - (col_gap * 2.0)).max(820.0);

            let left_w = (usable_w * 0.28).clamp(280.0, 380.0);
            let center_w = (usable_w * 0.30).clamp(270.0, 420.0);
            let right_w = (usable_w - left_w - center_w - (col_gap * 2.0)).max(360.0);

            egui::ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        if margin_x > 0.0 {
                            ui.add_space(margin_x);
                        }

                        // Left Column: Buttons & Profiles
                        ui.vertical(|ui| {
                            ui.set_width(left_w);
                            let profile = self.config.profiles.active_profile_mut();
                            self.buttons_panel.show(
                                ui,
                                &mut profile.buttons,
                                &mut self.mouse_view.selected_button,
                            );

                            ui.add_space(8.0);
                            self.profiles_panel.show(
                                ui,
                                &mut self.config.profiles,
                                self.device.battery_status,
                                &mut profile_switched,
                            );
                        });

                        ui.add_space(col_gap / 2.0);
                        ui.separator();
                        ui.add_space(col_gap / 2.0);

                        // Center Column: Mouse interactive representation
                        ui.vertical(|ui| {
                            ui.set_width(center_w);
                            self.mouse_view.show(ui, self.device.connection_type);
                        });

                        ui.add_space(col_gap / 2.0);
                        ui.separator();
                        ui.add_space(col_gap / 2.0);

                        // Right Column: DPI & Performance settings
                        ui.vertical(|ui| {
                            ui.set_width(right_w);
                            let profile = self.config.profiles.active_profile_mut();
                            self.dpi_panel.show(ui, &mut profile.sensor);
                            if self.dpi_panel.pending_apply {
                                self.dpi_panel.pending_apply = false;
                                trigger_apply = true;
                            }

                            ui.add_space(8.0);
                            self.perf_panel.show(
                                ui,
                                &mut profile.sensor,
                                &mut profile.power,
                                &mut profile.polling_rate,
                                &mut trigger_apply,
                            );
                        });

                        if margin_x > 0.0 {
                            ui.add_space(margin_x);
                        }
                    });
                });
        });

        // Macro Manager Dialog Modal
        if self.buttons_panel.open_macro_dialog {
            self.macro_dialog.show(
                ctx,
                &mut self.buttons_panel.open_macro_dialog,
                &mut self.config.macros,
                &mut self.device,
            );
        }

        if profile_switched {
            let _ = self.config.save();
            self.apply_current_configuration();
        } else if trigger_apply {
            self.apply_current_configuration();
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let _ = self.config.save();
    }
}
