use anyhow::{anyhow, Result as AnyResult};
use std::{env, fs, process::Command};
use which::which;

fn main() -> AnyResult<()> {
    let home_dir = env::home_dir().ok_or_else(|| anyhow!("home_dir is `None`"))?;
    let todo_path = home_dir.join(".todo/todos");
    let cwd = env::current_dir()?;
    let stripped = cwd.strip_prefix(home_dir).unwrap_or(&cwd);
    let cwd_todo = todo_path.join(stripped);
    let file = cwd_todo.join("todo.md");
    let editor = env::var("EDITOR").map_err(|_| anyhow!("EDITOR is not set"))?;
    fs::create_dir_all(&cwd_todo)?;
    Command::new(which(&editor).map_err(|_| anyhow!("Cannot find binary path for `{editor}`"))?)
        .arg(file)
        .spawn()?
        .wait()?;
    Ok(())
}
