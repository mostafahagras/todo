mod cli;
mod config;
mod remove;
mod sync;
mod update;
mod utils;
use crate::{
    cli::{Cli, Commands},
    config::{configure, load_config},
    remove::remove,
    sync::{sync, unsync},
    update::update,
    utils::{get_cwd_todo_dir, get_todo_file_path, resolve_editor},
};
use anyhow::{Result as AnyResult, anyhow};
use clap::Parser;
use std::{fs, io, process::Command};
use which::which;

fn main() -> AnyResult<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(command) => match command {
            Commands::Update => update(),
            Commands::Sync => sync(get_todo_file_path()?),
            Commands::Unsync => unsync(get_todo_file_path()?),
            Commands::List => {
                match fs::read_to_string(get_todo_file_path()?) {
                    Ok(content) => println!("{}", content.trim()),
                    Err(error) if error.kind() == io::ErrorKind::NotFound => {
                        return Err(anyhow!(
                            "❌ No todo file found for this directory. Run `todo` to create one."
                        ));
                    }
                    Err(error) => return Err(anyhow!("❌ Failed to read todo file: {error}")),
                }
                Ok(())
            }
            Commands::Config => {
                configure(true)?;
                Ok(())
            }
            Commands::Remove(args) => remove(args),
        },
        None => {
            let cwd_todo_dir = get_cwd_todo_dir()?;
            fs::create_dir_all(&cwd_todo_dir)
                .map_err(|e| anyhow!("❌ Failed to create todo directory: {e}"))?;
            let config = load_config()?;
            let config = match config {
                Some(config) => config,
                None => configure(false)?,
            };
            let editor = resolve_editor(config.editor)?;
            Command::new(
                which(&editor).map_err(|_| anyhow!("❌ Could not find the editor binary `{editor}`. Please check your config or PATH."))?,
            )
            .arg(get_todo_file_path()?)
            .spawn()
            .map_err(|e| anyhow!("❌ Failed to launch editor `{editor}`: {e}"))?
            .wait()
            .map_err(|e| anyhow!("❌ Editor process failed: {e}"))?;
            Ok(())
        }
    }?;
    Ok(())
}
