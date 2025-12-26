//! Windows wallpaper style and setter helpers.

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::Path;

use anyhow::{anyhow, Result};
use winreg::enums::{HKEY_CURRENT_USER, KEY_SET_VALUE};
use winreg::RegKey;
use windows::Win32::UI::WindowsAndMessaging::{
    SystemParametersInfoW, SPI_SETDESKWALLPAPER, SPIF_SENDCHANGE, SPIF_UPDATEINIFILE,
};

/// Windows wallpaper style modes.
#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum StyleMode {
    /// Fill the screen, cropping if needed.
    Fill,
    /// Fit entire image, preserving aspect.
    Fit,
    /// Stretch to fill without preserving aspect.
    Stretch,
    /// Tile the image across the screen.
    Tile,
    /// Center without scaling.
    Center,
    /// Span across multiple monitors.
    Span,
}

impl StyleMode {
    /// Fixed list of all supported style modes.
    pub const ALL: [StyleMode; 6] = [
        StyleMode::Fill,
        StyleMode::Fit,
        StyleMode::Stretch,
        StyleMode::Tile,
        StyleMode::Center,
        StyleMode::Span,
    ];

    /// English label used in the UI.
    pub fn label(&self) -> &'static str {
        match self {
            StyleMode::Fill => "Fill",
            StyleMode::Fit => "Fit",
            StyleMode::Stretch => "Stretch",
            StyleMode::Tile => "Tile",
            StyleMode::Center => "Center",
            StyleMode::Span => "Span",
        }
    }
}

/// Apply the Windows registry values for the selected style.
pub fn set_wallpaper_style(mode: StyleMode) -> Result<()> {
    let (style, tile) = match mode {
        StyleMode::Fill => ("10", "0"),
        StyleMode::Fit => ("6", "0"),
        StyleMode::Stretch => ("2", "0"),
        StyleMode::Tile => ("0", "1"),
        StyleMode::Center => ("0", "0"),
        StyleMode::Span => ("22", "0"),
    };
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let desktop = hkcu.open_subkey_with_flags("Control Panel\\Desktop", KEY_SET_VALUE)?;
    // These values map to Windows wallpaper styles (WallpaperStyle / TileWallpaper).
    desktop.set_value("WallpaperStyle", &style)?;
    desktop.set_value("TileWallpaper", &tile)?;
    Ok(())
}

/// Apply a BMP wallpaper via SystemParametersInfoW.
pub fn set_wallpaper(path: &Path) -> Result<()> {
    let wide_path = to_wide_null(path);
    unsafe {
        SystemParametersInfoW(
            SPI_SETDESKWALLPAPER,
            0,
            Some(wide_path.as_ptr() as *mut _),
            SPIF_UPDATEINIFILE | SPIF_SENDCHANGE,
        )
    }
    .map_err(|err| anyhow!("SystemParametersInfoW failed: {err}"))?;
    Ok(())
}

/// Convert a UTF-8 path to a wide null-terminated string.
fn to_wide_null(path: &Path) -> Vec<u16> {
    OsStr::new(path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
