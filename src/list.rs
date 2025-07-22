use crate::{
    cli::{ListArgs, ListSubcommand},
    utils::get_todo_file_path,
};
use anyhow::{anyhow, Result as AnyResult};
use regex::Regex;
use std::{fs, io};

pub enum ListMode {
    Raw,
    All,
    Checked,
    Unchecked,
}

impl ListMode {
    /// Returns `true` if the list mode is [`Raw`].
    ///
    /// [`Raw`]: ListMode::Raw
    #[must_use]
    pub fn is_raw(&self) -> bool {
        matches!(self, Self::Raw)
    }
}

impl From<ListArgs> for ListMode {
    fn from(value: ListArgs) -> Self {
        if value.raw && value.filter.is_some() {
            panic!("raw cannot be used with filters")
        }
        if value.raw {
            return Self::Raw;
        }
        match value.filter {
            Some(ListSubcommand::Checked) => ListMode::Checked,
            Some(ListSubcommand::Unchecked) => ListMode::Unchecked,
            None => ListMode::All,
        }
    }
}

pub fn list(mode: ListMode) -> AnyResult<()> {
    if mode.is_raw() {
        return list_raw();
    }
    let regex = match mode {
        ListMode::All => r"^\s*[-*+]? ?\[( |x)\](.+)$",
        ListMode::Checked => r"^\s*[-*+]? ?\[(x)\](.+)$",
        ListMode::Unchecked => r"^\s*[-*+]? ?\[( )\](.+)$",
        _ => unreachable!("handled by is_raw"),
    };
    let regex = Regex::new(regex).unwrap();
    let content = fs::read_to_string(get_todo_file_path()?)?;

    let todos: Vec<&str> = content
        .lines()
        .filter_map(|line| regex.captures(line).map(|_| line.trim_end()))
        .collect();

    if todos.is_empty() {
        match mode {
            ListMode::All => println!("No todos found."),
            ListMode::Checked => println!("No checked todos found."),
            ListMode::Unchecked => println!("No unchecked todos found."),
            _ => unreachable!("handled by is_raw"),
        };
        return Ok(());
    }

    for todo in todos {
        println!("{todo}");
    }
    Ok(())
}

fn list_raw() -> AnyResult<()> {
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
