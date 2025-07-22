mod cli;
mod config;
mod count;
mod delete;
mod list;
mod sync;
mod todo_ops;
mod update;
mod utils;
use crate::{
    cli::{Cli, Commands, ConfigSubcommand},
    config::{configure, load_config},
    delete::delete,
    sync::{sync, unsync},
    todo_ops::{check, remove, search, uncheck},
    update::update,
    utils::{get_config_path, get_cwd_todo_dir, get_todo_file_path, resolve_editor},
};
use anyhow::{anyhow, Result as AnyResult};
use clap::Parser;
use count::count;
use list::list;
use std::{fs, process::Command};
use which::which;

fn main() -> AnyResult<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(command) => match command {
            Commands::Update => update(),
            Commands::Sync => sync(get_todo_file_path()?),
            Commands::Unsync => unsync(get_todo_file_path()?),
            Commands::Check { query, all } => check(query.unwrap_or_default(), all),
            Commands::Search { query } => search(query.unwrap_or_default()),
            Commands::Uncheck { query, all } => uncheck(query.unwrap_or_default(), all),
            Commands::Remove { query, all } => remove(query.unwrap_or_default(), all),
            Commands::List(args) => list(args.into()),
            Commands::Count(args) => count(args.filter.into()),
            Commands::Config(args) => {
                match args.action {
                    Some(ConfigSubcommand::List) => {
                        let config_path = get_config_path()?;
                        if config_path.exists() {
                            println!("{}", fs::read_to_string(config_path)?.trim());
                        } else {
                            println!("No configuration file, try running `todo` or `todo config`");
                        }
                    }
                    None => {
                        configure(true)?;
                    }
                };
                Ok(())
            }
            Commands::Delete(args) => delete(args),
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
            .args(config.flags)
            .spawn()
            .map_err(|e| anyhow!("❌ Failed to launch editor `{editor}`: {e}"))?
            .wait()
            .map_err(|e| anyhow!("❌ Editor process failed: {e}"))?;
            Ok(())
        }
    }?;
    Ok(())
}
