use std::io;

use anyhow::{Context, Result};
use winreg::enums::{HKEY_CURRENT_USER, KEY_QUERY_VALUE, KEY_SET_VALUE};
use winreg::RegKey;

const RUN_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
const RUN_VALUE: &str = "WallpaperManager";

pub fn is_enabled() -> Result<bool> {
    let key = open_run_key(KEY_QUERY_VALUE)?;
    match key.get_value::<String, _>(RUN_VALUE) {
        Ok(_) => Ok(true),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(err) => Err(err).context("failed to read startup registry value"),
    }
}

pub fn enable() -> Result<()> {
    let key = open_run_key(KEY_SET_VALUE)?;
    let exe = std::env::current_exe().context("failed to resolve current executable")?;
    let command = format!("\"{}\"", exe.display());
    key.set_value(RUN_VALUE, &command)
        .context("failed to set startup registry value")?;
    Ok(())
}

pub fn disable() -> Result<()> {
    let key = open_run_key(KEY_SET_VALUE)?;
    match key.delete_value(RUN_VALUE) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(err).context("failed to remove startup registry value"),
    }
}

fn open_run_key(flags: u32) -> Result<RegKey> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    hkcu.open_subkey_with_flags(RUN_KEY, flags)
        .context("failed to open startup registry key")
}
