use anyhow::{anyhow, Result as AnyResult};
use inquire::{Select, Text};
use serde::{Deserialize, Serialize};
use std::{env, fs, io, process::exit};

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

#[derive(Debug, Serialize)]
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

pub fn load_config() -> AnyResult<Option<Config>> {
    let home_dir = env::home_dir().ok_or_else(|| anyhow!("home_dir is `None`"))?;
    let config_path = home_dir.join(".todo/config.toml");
    let config = fs::read_to_string(config_path);
    match config {
        Ok(config_str) => Ok(Some(
            toml::from_str::<RawConfig>(&config_str)
                .unwrap_or_default()
                .into(),
        )),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(err.into()),
    }
}

pub fn init_config() -> AnyResult<Config> {
    let options = vec!["1) Use default config", "2) Customize config", "3) Cancel"];
    let choice = Select::new("Looks like you have no config yet...", options)
        .prompt()
        .map_err(|e| anyhow!("Prompt failed: {}", e))?;
    let config = match choice.chars().next().unwrap() {
        '1' => RawConfig::default().into(),
        '2' => Config {
            filename: Text::new("Filename:").with_default("todo").prompt()?,
            extension: Text::new("Extension:").with_default(".md").prompt()?,
            editor: Text::new("Editor:").with_default("$EDITOR").prompt()?,
        },
        '3' => exit(0),
        _ => unreachable!("no such option"),
    };

    let home_dir = env::home_dir().ok_or_else(|| anyhow!("home_dir is `None`"))?;
    let config_path = home_dir.join(".todo/config.toml");
    fs::create_dir_all(config_path.parent().unwrap())?;
    fs::write(&config_path, toml::to_string(&config)?)?;
    println!("Saved config to {}", config_path.display());
    Ok(config)
}
