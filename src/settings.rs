//! Persistence model and configuration IO.

use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::i18n::Language;
use crate::wallpaper::StyleMode;

/// File name used under the per-user config directory.
const SETTINGS_FILE: &str = "settings.json";

/// Serializable representation of a folder entry.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FolderSetting {
    /// Absolute folder path stored on disk.
    pub path: String,
    /// Whether subfolders should be scanned.
    pub include_subfolders: bool,
}

/// Theme options exposed in the UI.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ThemeMode {
    /// Light egui visuals.
    Light,
    /// Dark egui visuals (default).
    #[serde(other)]
    Dark,
}

impl Default for ThemeMode {
    fn default() -> Self {
        // Default to dark to match the app's preferred look.
        ThemeMode::Dark
    }
}

/// Settings persisted to `settings.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    /// Whether the app starts with Windows.
    pub run_on_startup: bool,
    /// Whether to minimize to tray when started at boot.
    pub minimize_to_tray_on_start: bool,
    /// Folder list used for image discovery.
    pub folders: Vec<FolderSetting>,
    /// Optional single image override.
    pub single_image: Option<String>,
    /// Auto-rotate portrait images.
    pub auto_rotate: bool,
    /// Random vs sequential selection.
    pub random_order: bool,
    /// Slideshow interval in seconds.
    pub interval_secs: u64,
    /// UI language selection.
    pub language: Language,
    /// Windows wallpaper style.
    pub style: StyleMode,
    /// Whether the slideshow should resume on startup.
    pub running: bool,
    /// Selected UI theme.
    pub theme: ThemeMode,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            run_on_startup: false,
            minimize_to_tray_on_start: false,
            folders: Vec::new(),
            single_image: None,
            auto_rotate: true,
            random_order: true,
            interval_secs: 600,
            language: Language::En,
            style: StyleMode::Fill,
            running: false,
            theme: ThemeMode::Dark,
        }
    }
}

/// Resolve the per-user settings file path.
/// Build the settings path and ensure the directory exists.
fn settings_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("dev", "wallpaper_manager", "wallpaper_manager")
        .ok_or_else(|| anyhow!("cannot determine config directory"))?;
    let config_dir = proj_dirs.config_dir();
    fs::create_dir_all(config_dir)?;
    Ok(config_dir.join(SETTINGS_FILE))
}

/// Load settings from disk, returning defaults when missing.
pub fn load() -> AppSettings {
    let path = match settings_path() {
        Ok(path) => path,
        Err(_) => return AppSettings::default(),
    };
    let contents = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(_) => return AppSettings::default(),
    };
    serde_json::from_str(&contents).unwrap_or_default()
}

/// Persist settings to disk as pretty JSON.
pub fn save(settings: &AppSettings) -> Result<()> {
    let path = settings_path()?;
    // Keep settings on disk so startup options are available before the GUI initializes.
    let contents = serde_json::to_string_pretty(settings)?;
    fs::write(path, contents)?;
    Ok(())
}
