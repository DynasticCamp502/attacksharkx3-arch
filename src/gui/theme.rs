//! Visual theme and design system matching the original Attack Shark X3 Windows application.

use eframe::egui::{self, Color32, Rounding, Stroke, Vec2};

pub const COLOR_BG_DARK: Color32 = Color32::from_rgb(0x14, 0x14, 0x14);
pub const COLOR_PANEL_BG: Color32 = Color32::from_rgb(0x1F, 0x1F, 0x1F);
pub const COLOR_CARD_BG: Color32 = Color32::from_rgb(0x27, 0x27, 0x27);
pub const COLOR_ACCENT_RED: Color32 = Color32::from_rgb(0xE2, 0x21, 0x1C);
pub const COLOR_ACCENT_RED_HOVER: Color32 = Color32::from_rgb(0xFF, 0x3B, 0x30);
pub const COLOR_BORDER: Color32 = Color32::from_rgb(0x38, 0x38, 0x38);
pub const COLOR_BORDER_ACTIVE: Color32 = Color32::from_rgb(0xE2, 0x21, 0x1C);

pub const COLOR_TEXT_PRIMARY: Color32 = Color32::from_rgb(0xFF, 0xFF, 0xFF);
pub const COLOR_TEXT_SECONDARY: Color32 = Color32::from_rgb(0xAA, 0xAA, 0xAA);
pub const COLOR_TEXT_MUTED: Color32 = Color32::from_rgb(0x70, 0x70, 0x70);

pub const COLOR_CONNECTED_GREEN: Color32 = Color32::from_rgb(0x34, 0xC7, 0x59);
pub const COLOR_BATTERY_GREEN: Color32 = Color32::from_rgb(0x30, 0xD1, 0x58);

pub fn apply_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.visuals.dark_mode = true;
    style.visuals.override_text_color = Some(COLOR_TEXT_PRIMARY);
    style.visuals.panel_fill = COLOR_BG_DARK;
    style.visuals.window_fill = COLOR_PANEL_BG;
    style.visuals.extreme_bg_color = COLOR_BG_DARK;
    style.visuals.faint_bg_color = COLOR_CARD_BG;

    style.visuals.widgets.noninteractive.bg_fill = COLOR_PANEL_BG;
    style.visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, COLOR_BORDER);
    style.visuals.widgets.noninteractive.rounding = Rounding::same(4.0);

    style.visuals.widgets.inactive.bg_fill = COLOR_CARD_BG;
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, COLOR_BORDER);
    style.visuals.widgets.inactive.rounding = Rounding::same(4.0);

    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(0x35, 0x35, 0x35);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, COLOR_ACCENT_RED_HOVER);
    style.visuals.widgets.hovered.rounding = Rounding::same(4.0);

    style.visuals.widgets.active.bg_fill = COLOR_ACCENT_RED;
    style.visuals.widgets.active.bg_stroke = Stroke::new(1.5, COLOR_ACCENT_RED);
    style.visuals.widgets.active.rounding = Rounding::same(4.0);

    style.spacing.item_spacing = Vec2::new(8.0, 8.0);
    style.spacing.button_padding = Vec2::new(12.0, 6.0);

    ctx.set_style(style);
}

pub fn section_header(ui: &mut egui::Ui, title: &str) {
    ui.horizontal(|ui| {
        let (rect, _) = ui.allocate_exact_size(Vec2::new(4.0, 16.0), egui::Sense::hover());
        ui.painter().rect_filled(rect, Rounding::same(2.0), COLOR_ACCENT_RED);
        ui.label(
            egui::RichText::new(title)
                .color(COLOR_TEXT_PRIMARY)
                .strong()
                .size(13.5),
        );
    });
    ui.add_space(4.0);
}
