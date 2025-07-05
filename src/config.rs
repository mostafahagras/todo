use crate::utils::{get_config_path, get_todo_path, update_todos};
use anyhow::{anyhow, Result as AnyResult};
use inquire::{Select, Text};
use serde::{Deserialize, Serialize};
use std::{fs, io, process::exit};

#[derive(Debug, Deserialize)]
struct RawConfig {
    filename: Option<String>,
    extension: Option<String>,
    editor: Option<String>,
    flags: Option<Vec<String>>,
}

impl Default for RawConfig {
    fn default() -> Self {
        Self {
            filename: Some("todo".into()),
            extension: Some(".md".into()),
            editor: Some("$EDITOR".into()),
            flags: Some(Vec::new()),
        }
    }
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct Config {
    pub filename: String,
    pub extension: String,
    pub editor: String,
    pub flags: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        RawConfig::default().into()
    }
}

impl Config {
    fn names_changed(old: &Self, new: &Self) -> bool {
        old.filename != new.filename || old.extension != new.extension
    }
}

impl From<RawConfig> for Config {
    fn from(value: RawConfig) -> Self {
        Self {
            filename: value.filename.unwrap_or("todo".into()),
            extension: value.extension.unwrap_or(".md".into()),
            editor: value.editor.unwrap_or("$EDITOR".into()),
            flags: value.flags.unwrap_or_default(),
        }
    }
}

pub fn load_config() -> AnyResult<Option<Config>> {
    let config_path = get_config_path()?;
    let config = fs::read_to_string(&config_path);
    match config {
        Ok(config_str) => Ok(Some(
            toml::from_str::<RawConfig>(&config_str)
                .unwrap_or_default()
                .into(),
        )),
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(err) => Err(anyhow!(
            "❌ Failed to read config file at {}: {err}",
            config_path.display()
        )),
    }
}

pub fn configure(user_triggered: bool) -> AnyResult<Config> {
    let options = vec!["1) Use default config", "2) Customize config", "3) Cancel"];
    let prompt = if user_triggered {
        "Select an option"
    } else {
        "Looks like you have no config yet..."
    };
    let choice = Select::new(prompt, options)
        .prompt()
        .map_err(|e| anyhow!("❌ Prompt failed: {e}"))?;
    let config = match choice.chars().next().unwrap() {
        '1' => Config::default(),
        '2' => {
            let old_config = if user_triggered {
                load_config()?.unwrap_or_default()
            } else {
                Default::default()
            };
            Config {
                filename: Text::new("Filename:")
                    .with_default(&old_config.filename)
                    .prompt()
                    .map_err(|e| anyhow!("❌ Failed to get filename: {e}"))?,
                extension: Text::new("Extension:")
                    .with_default(&old_config.extension)
                    .prompt()
                    .map_err(|e| anyhow!("❌ Failed to get extension: {e}"))?,
                editor: Text::new("Editor:")
                    .with_default(&old_config.editor)
                    .prompt()
                    .map_err(|e| anyhow!("❌ Failed to get editor: {e}"))?,
                flags: Text::new("Editor flags:")
                    .with_default(&old_config.flags.join(" "))
                    .prompt()
                    .map(|s| s.split_whitespace().map(|s| s.to_string()).collect())
                    .map_err(|e| anyhow!("Failed to get editor flags: {e}"))?,
            }
        }
        '3' => exit(0),
        _ => unreachable!("❌ No such option selected."),
    };

    let old_config = load_config()?;
    if let Some(ref old_config) = old_config {
        if config == *old_config {
            println!("ℹ The config wasn't changed");
            return Ok(config);
        }
    }
    let config_path = get_config_path()?;
    fs::create_dir_all(config_path.parent().unwrap())
        .map_err(|e| anyhow!("❌ Failed to create config directory: {e}"))?;
    fs::write(&config_path, toml::to_string(&config)?)
        .map_err(|e| anyhow!("❌ Failed to write config file: {e}"))?;
    println!("✅ Saved config to {}", config_path.display());
    if old_config.is_some() && Config::names_changed(&config, &old_config.unwrap()) {
        let todos_path = get_todo_path()?;
        if todos_path.exists() {
            match update_todos(todos_path, &config) {
                Ok(_) => println!("✅ Updated todos"),
                Err(err) => {
                    return Err(anyhow!("❌ Failed to update todos: {err:?}"));
                }
            }
        }
    }
    Ok(config)
}
