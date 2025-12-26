//! Application runtime state derived from persisted settings.

use std::path::PathBuf;

use crate::i18n::Language;
use crate::image_ops::FolderSource;
use crate::settings::{AppSettings, FolderSetting, ThemeMode};
use crate::wallpaper::StyleMode;

/// In-memory state that drives UI rendering and slideshow behavior.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Ordered list of folder inputs.
    pub folders: Vec<FolderSource>,
    /// Optional single-image override.
    pub single_image: Option<PathBuf>,
    /// Whether portrait images should be rotated to landscape.
    pub auto_rotate: bool,
    /// Whether to choose images randomly instead of sequential order.
    pub random_order: bool,
    /// Slideshow interval in seconds.
    pub interval_secs: u64,
    /// Current UI language.
    pub language: Language,
    /// Current Windows wallpaper style.
    pub style: StyleMode,
    /// Current theme selection.
    pub theme: ThemeMode,
    /// Whether the slideshow should be running.
    pub running: bool,
}

impl AppState {
    /// Build the runtime state from persisted settings.
    pub fn from_settings(settings: &AppSettings) -> Self {
        let folders = settings
            .folders
            .iter()
            .map(|folder| FolderSource {
                path: PathBuf::from(&folder.path),
                include_subfolders: folder.include_subfolders,
            })
            .collect();
        let single_image = settings
            .single_image
            .as_ref()
            .map(|path| PathBuf::from(path));

        Self {
            folders,
            single_image,
            auto_rotate: settings.auto_rotate,
            random_order: settings.random_order,
            interval_secs: settings.interval_secs,
            language: settings.language,
            style: settings.style,
            theme: settings.theme,
            running: settings.running,
        }
    }

    /// Copy the runtime state back into settings for persistence.
    pub fn apply_to_settings(&self, settings: &mut AppSettings) {
        settings.folders = self
            .folders
            .iter()
            .map(|folder| FolderSetting {
                path: folder.path.to_string_lossy().to_string(),
                include_subfolders: folder.include_subfolders,
            })
            .collect();
        settings.single_image = self
            .single_image
            .as_ref()
            .map(|path| path.to_string_lossy().to_string());
        settings.auto_rotate = self.auto_rotate;
        settings.random_order = self.random_order;
        settings.interval_secs = self.interval_secs;
        settings.language = self.language;
        settings.style = self.style;
        settings.theme = self.theme;
        settings.running = self.running;
    }
}
