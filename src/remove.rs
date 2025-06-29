use anyhow::{anyhow, Result as AnyResult};
use inquire::prompt_confirmation;
use std::{env, fs};

use crate::{cli::RemoveArgs, config::load_config};

pub fn remove(args: RemoveArgs) -> AnyResult<()> {
    let home_dir = env::home_dir().ok_or_else(|| anyhow!("home_dir is `None`"))?;
    let todo_path = home_dir.join(".todo/todos");
    let cwd = env::current_dir()?;
    let stripped = cwd.strip_prefix(home_dir).unwrap_or(&cwd);
    let cwd_todo_dir = todo_path.join(stripped);

    let config = load_config()?.unwrap_or_default();
    let file = config.filename + &config.extension;
    let file_path = cwd_todo_dir.join(&file);

    if args.all {
        prompt_confirmation("Are you sure you want to delete all your todos")?
            .then(|| fs::remove_dir_all(todo_path));
    } else if args.recurse {
        prompt_confirmation(
            "Are you sure you want to delete todo files in this directory and all subdirectories?",
        )?
        .then(|| fs::remove_dir_all(cwd_todo_dir));
    } else {
        prompt_confirmation("Are you sure you want to delete the todo for this folder")?
            .then(|| fs::remove_file(file_path));
    }
    Ok(())
}
