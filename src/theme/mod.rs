//! Theme management for the egui-based UI.

use eframe::egui;

use crate::settings::ThemeMode;

/// Apply the selected theme to the egui context.
pub fn apply_theme(ctx: &egui::Context, theme: ThemeMode) {
    let visuals = match theme {
        ThemeMode::Light => egui::Visuals::light(),
        ThemeMode::Dark => egui::Visuals::dark(),
    };
    ctx.set_visuals(visuals);
}
