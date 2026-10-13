//! Central column displaying interactive representation of the Attack Shark X3 mouse.

use eframe::egui::{self, Color32, Pos2, Rect, Rounding, Stroke, Vec2};
use crate::device::ConnectionType;
use crate::gui::theme::*;

pub struct MouseView {
    pub selected_button: Option<usize>,
}

impl Default for MouseView {
    fn default() -> Self {
        Self { selected_button: None }
    }
}

impl MouseView {
    pub fn show(&mut self, ui: &mut egui::Ui, conn: ConnectionType) {
        ui.vertical_centered(|ui| {
            // Connection Indicator Bar
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Link:").color(COLOR_TEXT_MUTED).size(12.0));
                let (badge_color, badge_text) = match conn {
                    ConnectionType::Wireless24G => (COLOR_CONNECTED_GREEN, "2.4G Wireless"),
                    ConnectionType::WiredUsb => (COLOR_CONNECTED_GREEN, "Wired USB"),
                    ConnectionType::PermissionDenied => (COLOR_ACCENT_RED, "Access Denied (udev rule needed)"),
                    ConnectionType::Disconnected => (COLOR_TEXT_MUTED, "Not Connected"),
                };
                ui.label(
                    egui::RichText::new(badge_text)
                        .color(badge_color)
                        .strong()
                        .size(12.0),
                );
            });
            ui.add_space(12.0);

            // Responsive mouse sizing based on available width
            let col_w = ui.available_width();
            let scale = (col_w / 260.0).clamp(0.95, 1.35);
            let mouse_w = 175.0 * scale;
            let mouse_h = 315.0 * scale;

            let available_size = Vec2::new(col_w, mouse_h + 30.0);
            let (rect, response) = ui.allocate_exact_size(available_size, egui::Sense::click_and_drag());
            let painter = ui.painter_at(rect);

            // Draw Mouse Silhouette
            let mouse_body = Rect::from_center_size(rect.center(), Vec2::new(mouse_w, mouse_h));
            painter.rect_filled(
                mouse_body,
                Rounding {
                    nw: 50.0 * scale,
                    ne: 50.0 * scale,
                    sw: 65.0 * scale,
                    se: 65.0 * scale,
                },
                Color32::from_rgb(0x1B, 0x1B, 0x1B),
            );
            painter.rect_stroke(
                mouse_body,
                Rounding {
                    nw: 50.0 * scale,
                    ne: 50.0 * scale,
                    sw: 65.0 * scale,
                    se: 65.0 * scale,
                },
                Stroke::new(1.5_f32, COLOR_BORDER),
            );

            // Center scroll wheel
            let wheel_rect = Rect::from_center_size(
                Pos2::new(mouse_body.center().x, mouse_body.min.y + (75.0 * scale)),
                Vec2::new(20.0 * scale, 48.0 * scale),
            );
            let wheel_hover = self.selected_button == Some(3);
            painter.rect_filled(
                wheel_rect,
                Rounding::same(6.0 * scale),
                if wheel_hover { COLOR_ACCENT_RED } else { Color32::from_rgb(0x35, 0x35, 0x35) },
            );
            painter.rect_stroke(wheel_rect, Rounding::same(6.0 * scale), Stroke::new(1.0_f32, COLOR_BORDER));

            // Left Button (1)
            let left_btn = Rect::from_min_max(
                Pos2::new(mouse_body.min.x + (8.0 * scale), mouse_body.min.y + (12.0 * scale)),
                Pos2::new(mouse_body.center().x - (14.0 * scale), mouse_body.min.y + (130.0 * scale)),
            );
            let left_hover = self.selected_button == Some(1);
            painter.rect_stroke(
                left_btn,
                Rounding { nw: 36.0 * scale, ne: 4.0 * scale, sw: 4.0 * scale, se: 4.0 * scale },
                Stroke::new(if left_hover { 2.0_f32 } else { 1.0_f32 }, if left_hover { COLOR_ACCENT_RED } else { COLOR_BORDER }),
            );

            // Right Button (2)
            let right_btn = Rect::from_min_max(
                Pos2::new(mouse_body.center().x + (14.0 * scale), mouse_body.min.y + (12.0 * scale)),
                Pos2::new(mouse_body.max.x - (8.0 * scale), mouse_body.min.y + (130.0 * scale)),
            );
            let right_hover = self.selected_button == Some(2);
            painter.rect_stroke(
                right_btn,
                Rounding { nw: 4.0 * scale, ne: 36.0 * scale, sw: 4.0 * scale, se: 4.0 * scale },
                Stroke::new(if right_hover { 2.0_f32 } else { 1.0_f32 }, if right_hover { COLOR_ACCENT_RED } else { COLOR_BORDER }),
            );

            // DPI Switch Button (center, below wheel)
            let dpi_btn = Rect::from_center_size(
                Pos2::new(mouse_body.center().x, mouse_body.min.y + (120.0 * scale)),
                Vec2::new(18.0 * scale, 22.0 * scale),
            );
            painter.rect_filled(dpi_btn, Rounding::same(4.0 * scale), Color32::from_rgb(0x28, 0x28, 0x28));
            painter.rect_stroke(dpi_btn, Rounding::same(4.0 * scale), Stroke::new(1.0_f32, COLOR_BORDER));

            // Forward Button (4) - Left side
            let fwd_btn = Rect::from_min_max(
                Pos2::new(mouse_body.min.x - (6.0 * scale), mouse_body.min.y + (115.0 * scale)),
                Pos2::new(mouse_body.min.x + (3.0 * scale), mouse_body.min.y + (145.0 * scale)),
            );
            let fwd_hover = self.selected_button == Some(4);
            painter.rect_filled(
                fwd_btn,
                Rounding::same(3.0 * scale),
                if fwd_hover { COLOR_ACCENT_RED } else { Color32::from_rgb(0x3A, 0x3A, 0x3A) },
            );

            // Backward Button (5) - Left side
            let back_btn = Rect::from_min_max(
                Pos2::new(mouse_body.min.x - (6.0 * scale), mouse_body.min.y + (155.0 * scale)),
                Pos2::new(mouse_body.min.x + (3.0 * scale), mouse_body.min.y + (185.0 * scale)),
            );
            let back_hover = self.selected_button == Some(5);
            painter.rect_filled(
                back_btn,
                Rounding::same(3.0 * scale),
                if back_hover { COLOR_ACCENT_RED } else { Color32::from_rgb(0x3A, 0x3A, 0x3A) },
            );

            // Attack Shark logo mark on the palm rest
            painter.text(
                Pos2::new(mouse_body.center().x, mouse_body.center().y + (75.0 * scale)),
                egui::Align2::CENTER_CENTER,
                "ATTACK SHARK",
                egui::FontId::proportional(11.0 * scale),
                Color32::from_rgb(0x55, 0x55, 0x55),
            );
            painter.text(
                Pos2::new(mouse_body.center().x, mouse_body.center().y + (93.0 * scale)),
                egui::Align2::CENTER_CENTER,
                "X3",
                egui::FontId::proportional(14.0 * scale),
                COLOR_ACCENT_RED,
            );

            if response.clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    if left_btn.contains(pos) { self.selected_button = Some(1); }
                    else if right_btn.contains(pos) { self.selected_button = Some(2); }
                    else if wheel_rect.contains(pos) { self.selected_button = Some(3); }
                    else if fwd_btn.contains(pos) { self.selected_button = Some(4); }
                    else if back_btn.contains(pos) { self.selected_button = Some(5); }
                    else { self.selected_button = None; }
                }
            }
        });
    }
}
