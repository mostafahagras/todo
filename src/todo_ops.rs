use crate::utils::get_todo_file_path;
use anyhow::Result as AnyResult;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use inquire::MultiSelect;
use regex::Regex;
use std::fs::{self, write};

pub fn check(query: String) -> AnyResult<()> {
    let content = fs::read_to_string(get_todo_file_path()?)?;
    let todo_regex = Regex::new(r"^\s*[-*+]? ?\[ \](.+)$").unwrap();
    let matcher = SkimMatcherV2::default();

    let todos: Vec<(usize, &str, String)> = content
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            todo_regex.captures(line).map(|caps| {
                let text = caps.get(1).unwrap().as_str().trim().to_string();
                (i, line, text)
            })
        })
        .collect();

    if todos.is_empty() {
        println!("No unchecked todos found.");
        return Ok(());
    }

    if query.is_empty() {
        println!("No query entered. Aborting.");
        return Ok(());
    }

    let mut scored: Vec<_> = todos
        .iter()
        .filter_map(|(i, _line, text)| {
            matcher
                .fuzzy_match(text, &query)
                .map(|score| (score, *i, text))
        })
        .collect();

    if scored.is_empty() {
        println!("No matching todos.");
        return Ok(());
    }

    scored.sort_by(|a, b| b.0.cmp(&a.0));
    let best_score = scored[0].0;

    let best_matches: Vec<_> = scored
        .into_iter()
        .filter(|(score, _, _)| *score == best_score)
        .collect();

    let to_check: Vec<usize> = if best_matches.len() == 1 {
        vec![best_matches[0].1]
    } else {
        let options: Vec<_> = best_matches
            .iter()
            .map(|(_, _, line)| line.to_string())
            .collect();

        let choices =
            MultiSelect::new("Multiple matches found. Select todo(s) to check:", options).prompt();

        match choices {
            Ok(selected) => selected
                .into_iter()
                .filter_map(|sel| {
                    best_matches
                        .iter()
                        .find(|(_, _, line)| **line == sel)
                        .map(|(_, idx, _)| *idx)
                })
                .collect(),
            Err(_) => vec![],
        }
    };

    if to_check.is_empty() {
        println!("No todos selected.");
        return Ok(());
    }

    let updated: Vec<String> = content
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if to_check.contains(&i) {
                line.replacen("[ ]", "[x]", 1)
            } else {
                line.to_string()
            }
        })
        .collect();

    write(get_todo_file_path()?, updated.join("\n"))?;
    println!("Checked {} todo(s).", to_check.len());
    Ok(())
}

pub fn uncheck(query: String) -> AnyResult<()> {
    let content = fs::read_to_string(get_todo_file_path()?)?;
    let todo_regex = Regex::new(r"^\s*[-*+]? ?\[x\](.+)$").unwrap();
    let matcher = SkimMatcherV2::default();

    let todos: Vec<(usize, &str, String)> = content
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            todo_regex.captures(line).map(|caps| {
                let text = caps.get(1).unwrap().as_str().trim().to_string();
                (i, line, text)
            })
        })
        .collect();

    if todos.is_empty() {
        println!("No checked todos found.");
        return Ok(());
    }

    if query.is_empty() {
        println!("No query entered. Aborting.");
        return Ok(());
    }

    let mut scored: Vec<_> = todos
        .iter()
        .filter_map(|(i, _line, text)| {
            matcher
                .fuzzy_match(text, &query)
                .map(|score| (score, *i, text))
        })
        .collect();

    if scored.is_empty() {
        println!("No matching todos.");
        return Ok(());
    }

    scored.sort_by(|a, b| b.0.cmp(&a.0));
    let best_score = scored[0].0;

    let best_matches: Vec<_> = scored
        .into_iter()
        .filter(|(score, _, _)| *score == best_score)
        .collect();

    let to_uncheck: Vec<usize> = if best_matches.len() == 1 {
        vec![best_matches[0].1]
    } else {
        let options: Vec<_> = best_matches
            .iter()
            .map(|(_, _, line)| line.to_string())
            .collect();

        let choices = MultiSelect::new(
            "Multiple matches found. Select todo(s) to uncheck:",
            options,
        )
        .prompt();

        match choices {
            Ok(selected) => selected
                .into_iter()
                .filter_map(|sel| {
                    best_matches
                        .iter()
                        .find(|(_, _, line)| **line == sel)
                        .map(|(_, idx, _)| *idx)
                })
                .collect(),
            Err(_) => vec![],
        }
    };

    if to_uncheck.is_empty() {
        println!("No todos selected.");
        return Ok(());
    }

    let updated: Vec<String> = content
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if to_uncheck.contains(&i) {
                line.replacen("[x]", "[ ]", 1)
            } else {
                line.to_string()
            }
        })
        .collect();

    write(get_todo_file_path()?, updated.join("\n"))?;
    println!("Unchecked {} todo(s).", to_uncheck.len());
    Ok(())
}

pub fn search(query: String) -> AnyResult<()> {
    let content = fs::read_to_string(get_todo_file_path()?)?;
    let todo_regex = Regex::new(r"^\s*[-*+]? ?\[( |x)\](.+)$").unwrap();
    let matcher = SkimMatcherV2::default();

    let todos: Vec<(&str, String)> = content
        .lines()
        .filter_map(|line| {
            todo_regex.captures(line).map(|caps| {
                let text = caps.get(2).unwrap().as_str().trim().to_string();
                (line.trim_end(), text)
            })
        })
        .collect();

    if todos.is_empty() {
        println!("No todos found.");
        return Ok(());
    }

    if query.is_empty() {
        println!("No query entered. Aborting.");
        return Ok(());
    }

    let scored: Vec<_> = todos
        .iter()
        .filter_map(|(line, text)| {
            matcher.fuzzy_indices(text, &query).map(|(score, indices)| {
                let offset = line.strip_suffix(text).unwrap_or_default().len();
                let indices = indices
                    .iter()
                    .map(|idx| idx + offset)
                    .collect::<Vec<usize>>();
                (score, indices, line)
            })
        })
        .collect();

    if scored.is_empty() {
        println!("No matching todos.");
        return Ok(());
    }

    for (_, indices, todo) in scored {
        println!("{}", highlight_indices(todo, &indices));
    }
    Ok(())
}

fn highlight_indices(input: &str, indices: &[usize]) -> String {
    use std::collections::HashSet;
    let indices_set: HashSet<_> = indices.iter().copied().collect();

    input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if indices_set.contains(&i) {
                format!("\x1b[1;31m{}\x1b[0m", c) // bold red
            } else {
                c.to_string()
            }
        })
        .collect::<String>()
}
