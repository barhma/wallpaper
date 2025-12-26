//! Windows startup registration via HKCU\\...\\Run.

use std::io;

use anyhow::{Context, Result};
use winreg::enums::{HKEY_CURRENT_USER, KEY_QUERY_VALUE, KEY_SET_VALUE};
use winreg::RegKey;

/// Registry key for per-user startup apps.
const RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
/// Registry value name for this app.
const RUN_VALUE: &str = "WallpaperManager";

/// Return true when the startup registry value is present.
pub fn is_enabled() -> Result<bool> {
    let key = open_run_key(KEY_QUERY_VALUE)?;
    match key.get_value::<String, _>(RUN_VALUE) {
        Ok(_) => Ok(true),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(err) => Err(err).context("failed to read startup registry value"),
    }
}

/// Register the current executable to run at login.
pub fn enable() -> Result<()> {
    let key = open_run_key(KEY_SET_VALUE)?;
    let exe = std::env::current_exe().context("failed to resolve current executable")?;
    let command = format!("\"{}\" --startup", exe.display());
    key.set_value(RUN_VALUE, &command)
        .context("failed to set startup registry value")?;
    Ok(())
}

/// Remove the startup registry value if present.
pub fn disable() -> Result<()> {
    let key = open_run_key(KEY_SET_VALUE)?;
    match key.delete_value(RUN_VALUE) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err).context("failed to remove startup registry value"),
    }
}

/// Open the HKCU Run key with the desired access flags.
fn open_run_key(flags: u32) -> Result<RegKey> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    hkcu.open_subkey_with_flags(RUN_KEY, flags)
        .context("failed to open startup registry key")
}
