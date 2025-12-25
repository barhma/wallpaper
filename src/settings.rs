use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::i18n::Language;
use crate::wallpaper::StyleMode;

const SETTINGS_FILE: &str = "settings.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FolderSetting {
    pub path: String,
    pub include_subfolders: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppSettings {
    pub run_on_startup: bool,
    pub minimize_to_tray_on_start: bool,
    pub folders: Vec<FolderSetting>,
    pub single_image: Option<String>,
    pub auto_rotate: bool,
    pub random_order: bool,
    pub interval_secs: u64,
    pub language: Language,
    pub style: StyleMode,
    pub running: bool,
    pub light_theme: bool,
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
            light_theme: true,
        }
    }
}

fn settings_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("dev", "wallpaper_manager", "wallpaper_manager")
        .ok_or_else(|| anyhow!("cannot determine config directory"))?;
    let config_dir = proj_dirs.config_dir();
    fs::create_dir_all(config_dir)?;
    Ok(config_dir.join(SETTINGS_FILE))
}

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

pub fn save(settings: &AppSettings) -> Result<()> {
    let path = settings_path()?;
    // Keep settings on disk so startup options are available before the GUI initializes.
    let contents = serde_json::to_string_pretty(settings)?;
    fs::write(path, contents)?;
    Ok(())
}
