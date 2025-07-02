use crate::{
    cli::RemoveArgs,
    utils::{get_cwd_todo_dir, get_todo_file_path, get_todo_path},
};
use anyhow::{anyhow, Result as AnyResult};
use inquire::prompt_confirmation;
use std::fs;

pub fn remove(args: RemoveArgs) -> AnyResult<()> {
    let todo_path = get_todo_path()?;
    let cwd_todo_dir = get_cwd_todo_dir()?;
    let file_path = get_todo_file_path()?;
    if args.all {
        if prompt_confirmation(
            "Are you sure you want to delete ALL your todos? This cannot be undone.",
        )? {
            fs::remove_dir_all(&todo_path)
                .map_err(|e| anyhow!("❌ Failed to remove all todos: {e}"))?;
            println!("✅ All todos deleted.");
        }
    } else if args.recurse {
        if prompt_confirmation(
            "Are you sure you want to delete todo files in this directory and all subdirectories? This cannot be undone.",
        )? {
            fs::remove_dir_all(&cwd_todo_dir)
                .map_err(|e| anyhow!("❌ Failed to remove todos recursively: {e}"))?;
            println!("✅ Todos deleted recursively in this directory and subdirectories.");
        }
    } else if prompt_confirmation("Are you sure you want to delete the todo for this folder?")? {
        fs::remove_file(&file_path).map_err(|e| anyhow!("❌ Failed to remove todo file: {e}"))?;
        println!("✅ Todo deleted for this folder.");
    }
    Ok(())
}
