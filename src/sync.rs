use anyhow::{anyhow, Ok, Result as AnyResult};
use std::{fs, path::Path};

pub fn sync<P, Q>(original: P, link: Q) -> AnyResult<()>
where
    P: AsRef<Path> + std::fmt::Debug,
    Q: AsRef<Path> + std::fmt::Debug,
{
    if !fs::exists(&original)? {
        println!("❌ there's no todo file for the current directory");
        return Ok(());
    }
    if fs::exists(&link)? {
        println!("⚠️ {link:?} is already synced with {original:?}");
        return Ok(());
    }
    fs::hard_link(original, link).map_err(|e| anyhow!("Error: {e}"))
}

pub fn unsync<P, Q>(original: P, link: Q) -> AnyResult<()>
where
    P: AsRef<Path> + std::fmt::Debug,
    Q: AsRef<Path> + std::fmt::Debug,
{
    if !fs::exists(&link)? {
        println!("⚠️ {original:?} is not synced");
        return Ok(());
    }
    fs::remove_file(link).map_err(|e| anyhow!("Error: {e}"))
}
