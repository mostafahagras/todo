use anyhow::{Result as AnyResult, anyhow};
use std::{fs, path::PathBuf};

pub fn sync(original: PathBuf) -> AnyResult<()> {
    let link = original
        .file_name()
        .ok_or_else(|| anyhow!("Invalid path"))?;
    if !fs::exists(&original)? {
        return Err(anyhow!("❌ there's no todo file for the current directory"));
    }
    if fs::exists(link)? {
        println!("⚠️ {link:?} is already synced with {original:?}");
        return Ok(());
    }
    fs::hard_link(&original, link).map_err(|e| anyhow!("Error: {e}"))
}

pub fn unsync(original: PathBuf) -> AnyResult<()> {
    let synced_path = original
        .file_name()
        .ok_or_else(|| anyhow!("Invalid path"))?;
    if !fs::exists(synced_path)? {
        println!("⚠️ {original:?} is not synced");
        return Ok(());
    }
    fs::remove_file(synced_path).map_err(|e| anyhow!("Error: {e}"))
}
