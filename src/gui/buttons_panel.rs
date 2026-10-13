//! Left column: Buttons mapping configuration panel.

use eframe::egui::{self, Color32, Vec2};
use crate::buttons::{available_actions, ButtonAction, ButtonConfig};
use crate::gui::theme::*;

pub struct ButtonsPanel {
    pub open_macro_dialog: bool,
}

impl Default for ButtonsPanel {
    fn default() -> Self {
        Self { open_macro_dialog: false }
    }
}

impl ButtonsPanel {
    pub fn show(
        &mut self,
        ui: &mut egui::Ui,
        btn_cfg: &mut ButtonConfig,
        selected_button: &mut Option<usize>,
    ) {
        section_header(ui, "Button Settings");

        egui::Frame::none()
            .fill(COLOR_PANEL_BG)
            .inner_margin(8.0)
            .rounding(4.0)
            .stroke(egui::Stroke::new(1.0, COLOR_BORDER))
            .show(ui, |ui| {
                ui.spacing_mut().item_spacing = Vec2::new(6.0, 8.0);

                render_button_row(ui, 1, "1 Left Click", &mut btn_cfg.button1_left, selected_button);
                render_button_row(ui, 2, "2 Right Click", &mut btn_cfg.button2_right, selected_button);
                render_button_row(ui, 3, "3 Middle Click", &mut btn_cfg.button3_middle, selected_button);
                render_button_row(ui, 4, "4 Forward", &mut btn_cfg.button4_forward, selected_button);
                render_button_row(ui, 5, "5 Backward", &mut btn_cfg.button5_backward, selected_button);
            });

        ui.add_space(10.0);
        if ui
            .add_sized(
                [ui.available_width(), 28.0],
                egui::Button::new(
                    egui::RichText::new("Macro Manager")
                        .color(COLOR_TEXT_PRIMARY)
                        .strong(),
                )
                .fill(COLOR_CARD_BG),
            )
            .clicked()
        {
            self.open_macro_dialog = true;
        }
    }
}

fn render_button_row(
    ui: &mut egui::Ui,
    idx: usize,
    label: &str,
    action: &mut ButtonAction,
    selected: &mut Option<usize>,
) {
    ui.horizontal(|ui| {
        let is_selected = *selected == Some(idx);
        let btn_color = if is_selected { COLOR_ACCENT_RED } else { COLOR_TEXT_SECONDARY };

        let num_btn = ui.add_sized(
            [22.0, 22.0],
            egui::Button::new(egui::RichText::new(format!("{}", idx)).color(Color32::WHITE).size(11.0))
                .fill(if is_selected { COLOR_ACCENT_RED } else { Color32::from_rgb(0x35, 0x35, 0x35) }),
        );
        if num_btn.clicked() {
            *selected = Some(idx);
        }

        ui.label(egui::RichText::new(label).color(btn_color).size(12.0));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            egui::ComboBox::from_id_salt(format!("btn_combo_{}", idx))
                .selected_text(action.label())
                .width(110.0)
                .show_ui(ui, |ui| {
                    for &act in available_actions() {
                        ui.selectable_value(action, act, act.label());
                    }
                });
        });
    });
}
