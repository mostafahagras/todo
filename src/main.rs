mod config;
use crate::config::load_config;
use anyhow::{anyhow, Result as AnyResult};
use std::{env, fs, process::Command};
use which::which;

fn main() -> AnyResult<()> {
    let home_dir = env::home_dir().ok_or_else(|| anyhow!("home_dir is `None`"))?;
    let todo_path = home_dir.join(".todo/todos");
    let cwd = env::current_dir()?;
    let stripped = cwd.strip_prefix(home_dir).unwrap_or(&cwd);
    let cwd_todo_dir = todo_path.join(stripped);

    let config = load_config()?;
    let file = config.filename + &config.extension;
    let file_path = cwd_todo_dir.join(file);

    let editor = if config.editor.starts_with("$") {
        let var = &config.editor[1..];
        env::var(var).map_err(|_| anyhow!("Environment variable `{var}` is not set"))
    } else {
        Ok(config.editor)
    }?;

    fs::create_dir_all(&cwd_todo_dir)?;
    Command::new(which(&editor).map_err(|_| anyhow!("Cannot find binary path for `{editor}`"))?)
        .arg(file_path)
        .spawn()?
        .wait()?;
    Ok(())
}
