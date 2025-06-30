use crate::config::{load_config, Config};
use anyhow::{anyhow, Result as AnyResult};
use std::{env, fs, path::PathBuf};

pub fn get_home_dir() -> AnyResult<PathBuf> {
    env::home_dir().ok_or_else(|| {
        anyhow!("❌ Could not determine your home directory. Please check your environment.")
    })
}

pub fn get_todo_path() -> AnyResult<PathBuf> {
    Ok(get_home_dir()?.join(".todo").join("todos"))
}

pub fn get_config_path() -> AnyResult<PathBuf> {
    Ok(get_home_dir()?.join(".todo").join("config.toml"))
}

pub fn get_cwd_todo_dir() -> AnyResult<PathBuf> {
    let home_dir = get_home_dir()?;
    let todo_path = get_todo_path()?;
    let cwd = env::current_dir()?;
    let stripped = cwd.strip_prefix(&home_dir).unwrap_or(&cwd);
    Ok(todo_path.join(stripped))
}

pub fn get_todo_file_path() -> AnyResult<PathBuf> {
    let config = load_config()?.unwrap_or_default();
    Ok(get_cwd_todo_dir()?.join(config.filename + &config.extension))
}

pub fn resolve_editor(editor: String) -> AnyResult<String> {
    if editor.starts_with("$") {
        let var = &editor[1..];
        env::var(var).map_err(|_| {
            anyhow!(
                "❌ Environment variable `{var}` is not set. Please set it or update your config."
            )
        })
    } else {
        Ok(editor)
    }
}

pub fn update_todos(dir: PathBuf, new_config: &Config) -> AnyResult<()> {
    let new_filename = new_config.filename.clone() + &new_config.extension;
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            update_todos(path, new_config)?;
        } else {
            fs::rename(
                &path,
                path.parent()
                    .ok_or_else(|| anyhow!("❌ Invalid path"))?
                    .join(&new_filename),
            )?;
        }
    }
    Ok(())
}
