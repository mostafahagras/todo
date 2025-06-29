mod cli;
mod config;
mod remove;
mod sync;
mod update;
use crate::{
    cli::{Cli, Commands},
    config::{configure, load_config},
    remove::remove,
    sync::{sync, unsync},
    update::update,
};
use anyhow::{anyhow, Ok, Result as AnyResult};
use clap::Parser;
use std::{env, fs, process::Command};
use which::which;

fn main() -> AnyResult<()> {
    let cli = Cli::parse();
    let home_dir = env::home_dir().ok_or_else(|| anyhow!("home_dir is `None`"))?;
    let todo_path = home_dir.join(".todo/todos");
    let cwd = env::current_dir()?;
    let stripped = cwd.strip_prefix(home_dir).unwrap_or(&cwd);
    let cwd_todo_dir = todo_path.join(stripped);

    let config = load_config()?;
    let mut configured = false;
    let config = match config {
        Some(_) => config.unwrap(),
        None => {
            let config = configure(false)?;
            configured = true;
            config
        }
    };
    let file = config.filename + &config.extension;
    let file_path = cwd_todo_dir.join(&file);

    let editor = if config.editor.starts_with("$") {
        let var = &config.editor[1..];
        env::var(var).map_err(|_| anyhow!("Environment variable `{var}` is not set"))
    } else {
        Ok(config.editor)
    }?;

    match cli.command {
        Some(command) => match command {
            Commands::Update => update(),
            Commands::Sync => sync(file_path, file),
            Commands::Unsync => unsync(file_path, file),
            Commands::List => {
                println!("{}", fs::read_to_string(file_path)?);
                Ok(())
            }
            Commands::Config => {
                if !configured {
                    configure(!configured)?;
                }
                Ok(())
            }
            Commands::Remove(args) => remove(args),
        },
        None => {
            fs::create_dir_all(&cwd_todo_dir)?;
            Command::new(
                which(&editor).map_err(|_| anyhow!("Cannot find binary path for `{editor}`"))?,
            )
            .arg(file_path)
            .spawn()?
            .wait()?;
            Ok(())
        }
    }?;
    Ok(())
}
