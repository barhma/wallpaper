//! Theme management for the egui-based UI.

use eframe::egui;

use crate::settings::ThemeMode;

/// Apply the selected theme to the egui context.
pub fn apply_theme(ctx: &egui::Context, theme: ThemeMode) {
    let mut visuals = match theme {
        ThemeMode::Light => beige_visuals(),
        ThemeMode::Dark => purple_visuals(),
    };

    visuals.widgets.noninteractive.rounding = egui::Rounding::same(4.0);
    visuals.widgets.inactive.rounding = egui::Rounding::same(4.0);
    visuals.widgets.hovered.rounding = egui::Rounding::same(4.0);
    visuals.widgets.active.rounding = egui::Rounding::same(4.0);
    visuals.widgets.open.rounding = egui::Rounding::same(4.0);

    let mut style = (*ctx.style()).clone();
    style.visuals = visuals;
    style.spacing.item_spacing = egui::vec2(4.0, 4.0);
    style.spacing.button_padding = egui::vec2(8.0, 3.0);
    style.spacing.menu_margin = egui::Margin::same(4.0);
    style.spacing.window_margin = egui::Margin::same(6.0);
    style.spacing.indent = 8.0;

    ctx.set_style(style);
}

fn beige_visuals() -> egui::Visuals {
    let mut visuals = egui::Visuals::light();
    let text = egui::Color32::from_rgb(72, 54, 34);
    let panel = egui::Color32::from_rgb(245, 236, 220);
    let window = egui::Color32::from_rgb(252, 246, 234);
    let subtle = egui::Color32::from_rgb(235, 223, 204);
    let strong = egui::Color32::from_rgb(214, 186, 145);
    let accent = egui::Color32::from_rgb(168, 122, 79);

    visuals.override_text_color = Some(text);
    visuals.panel_fill = panel;
    visuals.window_fill = window;
    visuals.faint_bg_color = subtle;
    visuals.extreme_bg_color = egui::Color32::from_rgb(229, 214, 192);
    visuals.code_bg_color = egui::Color32::from_rgb(241, 230, 211);
    visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(202, 182, 151));
    visuals.selection.bg_fill = accent;
    visuals.selection.stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
    visuals.hyperlink_color = egui::Color32::from_rgb(123, 92, 163);
    visuals.warn_fg_color = egui::Color32::from_rgb(180, 112, 42);
    visuals.error_fg_color = egui::Color32::from_rgb(180, 72, 56);

    set_widget_visuals(
        &mut visuals.widgets.noninteractive,
        subtle,
        subtle,
        egui::Color32::from_rgb(205, 189, 164),
        text,
    );
    set_widget_visuals(
        &mut visuals.widgets.inactive,
        egui::Color32::from_rgb(238, 228, 210),
        strong,
        egui::Color32::from_rgb(189, 163, 126),
        text,
    );
    set_widget_visuals(
        &mut visuals.widgets.hovered,
        egui::Color32::from_rgb(229, 212, 184),
        egui::Color32::from_rgb(212, 183, 142),
        egui::Color32::from_rgb(160, 122, 86),
        text,
    );
    set_widget_visuals(
        &mut visuals.widgets.active,
        egui::Color32::from_rgb(168, 122, 79),
        egui::Color32::from_rgb(168, 122, 79),
        egui::Color32::from_rgb(126, 88, 52),
        egui::Color32::from_rgb(255, 248, 239),
    );
    set_widget_visuals(
        &mut visuals.widgets.open,
        egui::Color32::from_rgb(223, 204, 173),
        egui::Color32::from_rgb(223, 204, 173),
        egui::Color32::from_rgb(160, 122, 86),
        text,
    );

    visuals
}

fn purple_visuals() -> egui::Visuals {
    let mut visuals = egui::Visuals::dark();
    let text = egui::Color32::from_rgb(235, 226, 255);
    let panel = egui::Color32::from_rgb(35, 26, 58);
    let window = egui::Color32::from_rgb(29, 21, 49);
    let subtle = egui::Color32::from_rgb(49, 38, 76);
    let strong = egui::Color32::from_rgb(88, 63, 136);
    let accent = egui::Color32::from_rgb(164, 118, 255);

    visuals.override_text_color = Some(text);
    visuals.panel_fill = panel;
    visuals.window_fill = window;
    visuals.faint_bg_color = subtle;
    visuals.extreme_bg_color = egui::Color32::from_rgb(57, 45, 84);
    visuals.code_bg_color = egui::Color32::from_rgb(42, 31, 66);
    visuals.window_stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(98, 76, 146));
    visuals.selection.bg_fill = accent;
    visuals.selection.stroke = egui::Stroke::new(1.0, egui::Color32::from_rgb(248, 242, 255));
    visuals.hyperlink_color = egui::Color32::from_rgb(206, 176, 255);
    visuals.warn_fg_color = egui::Color32::from_rgb(255, 198, 120);
    visuals.error_fg_color = egui::Color32::from_rgb(255, 134, 150);

    set_widget_visuals(
        &mut visuals.widgets.noninteractive,
        subtle,
        subtle,
        egui::Color32::from_rgb(88, 69, 125),
        text,
    );
    set_widget_visuals(
        &mut visuals.widgets.inactive,
        egui::Color32::from_rgb(62, 47, 95),
        strong,
        egui::Color32::from_rgb(111, 84, 170),
        text,
    );
    set_widget_visuals(
        &mut visuals.widgets.hovered,
        egui::Color32::from_rgb(84, 64, 128),
        egui::Color32::from_rgb(118, 87, 188),
        egui::Color32::from_rgb(171, 132, 255),
        text,
    );
    set_widget_visuals(
        &mut visuals.widgets.active,
        accent,
        accent,
        egui::Color32::from_rgb(206, 176, 255),
        egui::Color32::from_rgb(29, 21, 49),
    );
    set_widget_visuals(
        &mut visuals.widgets.open,
        egui::Color32::from_rgb(97, 73, 150),
        egui::Color32::from_rgb(97, 73, 150),
        egui::Color32::from_rgb(191, 159, 255),
        text,
    );

    visuals
}

fn set_widget_visuals(
    visuals: &mut egui::style::WidgetVisuals,
    bg_fill: egui::Color32,
    weak_bg_fill: egui::Color32,
    stroke_color: egui::Color32,
    text_color: egui::Color32,
) {
    visuals.bg_fill = bg_fill;
    visuals.weak_bg_fill = weak_bg_fill;
    visuals.bg_stroke = egui::Stroke::new(1.0, stroke_color);
    visuals.fg_stroke = egui::Stroke::new(1.0, text_color);
}
