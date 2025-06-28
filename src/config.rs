use anyhow::{anyhow, Result as AnyResult};
use serde::Deserialize;
use std::{env, fs};

#[derive(Debug, Deserialize)]
struct RawConfig {
    filename: Option<String>,
    extension: Option<String>,
    editor: Option<String>,
}

impl Default for RawConfig {
    fn default() -> Self {
        Self {
            filename: Some("todo".into()),
            extension: Some(".md".into()),
            editor: Some("$EDITOR".into()),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub filename: String,
    pub extension: String,
    pub editor: String,
}

impl From<RawConfig> for Config {
    fn from(value: RawConfig) -> Self {
        Self {
            filename: value.filename.unwrap_or("todo".into()),
            extension: value.extension.unwrap_or(".md".into()),
            editor: value.editor.unwrap_or("$EDITOR".into()),
        }
    }
}

pub fn load_config() -> AnyResult<Config> {
    let home_dir = env::home_dir().ok_or_else(|| anyhow!("home_dir is `None`"))?;
    let config_path = home_dir.join(".todo/config.toml");
    let config = fs::read_to_string(config_path).unwrap_or("".into());
    let config: RawConfig = toml::from_str(&config).unwrap_or_default();
    Ok(config.into())
}
