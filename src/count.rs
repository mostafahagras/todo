use crate::{cli::CountSubcommand, utils::get_todo_file_path};
use anyhow::Result as AnyResult;
use regex::Regex;
use std::fs;

#[derive(Debug)]
pub enum CountFilter {
    All,
    Checked,
    Unchecked,
}

impl From<Option<CountSubcommand>> for CountFilter {
    fn from(value: Option<CountSubcommand>) -> Self {
        match value {
            Some(CountSubcommand::Checked) => CountFilter::Checked,
            Some(CountSubcommand::Unchecked) => CountFilter::Unchecked,
            None => CountFilter::All,
        }
    }
}

pub fn count(filter: CountFilter) -> AnyResult<()> {
    let regex = match filter {
        CountFilter::All => r"^\s*[-*+]? ?\[( |x)\](.+)$",
        CountFilter::Checked => r"^\s*[-*+]? ?\[(x)\](.+)$",
        CountFilter::Unchecked => r"^\s*[-*+]? ?\[( )\](.+)$",
    };
    let regex = Regex::new(regex).unwrap();
    let content = fs::read_to_string(get_todo_file_path()?)?;

    let todos: Vec<(&str, bool)> = content
        .lines()
        .filter_map(|line| {
            regex.captures(line).map(|caps| {
                let checked = caps.get(1).unwrap().as_str() == "x";
                (line.trim_end(), checked)
            })
        })
        .collect();

    if todos.is_empty() {
        match filter {
            CountFilter::All => println!("No todos found."),
            CountFilter::Checked => println!("No checked todos found."),
            CountFilter::Unchecked => println!("No unchecked todos found."),
        };
        return Ok(());
    }

    match filter {
        CountFilter::All => {
            let all = todos.len();
            let checked = todos
                .iter()
                .filter(|(_, checked)| *checked)
                .collect::<Vec<_>>()
                .len();
            println!("Total: {all}");
            println!("  Checked: {checked}");
            println!("  Unchecked: {}", all - checked);
        }
        _ => println!("{}", todos.len()),
    }
    Ok(())
}
