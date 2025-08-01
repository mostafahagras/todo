use crate::utils::{get_todo_file_path, highlight_indices};
use anyhow::Result as AnyResult;
use crossterm::{
    cursor::{self, Hide, Show},
    event::{read, Event, KeyCode},
    execute,
    style::{Print, Stylize},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use inquire::MultiSelect;
use regex::Regex;
use std::fs::{self, write};
use std::io::{stdout, Write};
use unicode_width::UnicodeWidthStr;

pub fn check(query: String, all: bool) -> AnyResult<()> {
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

    let to_check: Vec<usize> = if all {
        todos.iter().map(|(i, _, _)| *i).collect()
    } else if query.is_empty() {
        let options: Vec<_> = todos.iter().map(|(_, _, text)| text.clone()).collect();
        let choices = MultiSelect::new("Select todo(s) to check:", options).prompt();
        match choices {
            Ok(selected) => selected
                .into_iter()
                .filter_map(|sel| {
                    todos
                        .iter()
                        .find(|(_, _, text)| *text == sel)
                        .map(|(i, _, _)| *i)
                })
                .collect(),
            Err(_) => vec![],
        }
    } else {
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

        if best_matches.len() == 1 {
            vec![best_matches[0].1]
        } else {
            let options: Vec<_> = best_matches
                .iter()
                .map(|(_, _, line)| line.to_string())
                .collect();

            let choices =
                MultiSelect::new("Multiple matches found. Select todo(s) to check:", options)
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

pub fn uncheck(query: String, all: bool) -> AnyResult<()> {
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

    let to_uncheck: Vec<usize> = if all {
        todos.iter().map(|(i, _, _)| *i).collect()
    } else if query.is_empty() {
        let options: Vec<_> = todos.iter().map(|(_, _, text)| text.clone()).collect();
        let choices = MultiSelect::new("Select todo(s) to uncheck:", options).prompt();
        match choices {
            Ok(selected) => selected
                .into_iter()
                .filter_map(|sel| {
                    todos
                        .iter()
                        .find(|(_, _, text)| *text == sel)
                        .map(|(i, _, _)| *i)
                })
                .collect(),
            Err(_) => vec![],
        }
    } else {
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

        if best_matches.len() == 1 {
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

pub fn remove(query: String, all: bool) -> AnyResult<()> {
    let content = fs::read_to_string(get_todo_file_path()?)?;
    let todo_regex = Regex::new(r"^\s*[-*+]? ?\[[ x]\](.+)$").unwrap();
    let matcher = SkimMatcherV2::default();

    let todos: Vec<(usize, String)> = content
        .lines()
        .enumerate()
        .filter_map(|(i, line)| {
            todo_regex.captures(line).map(|caps| {
                let text = caps.get(1).unwrap().as_str().trim().to_string();
                (i, text)
            })
        })
        .collect();

    if todos.is_empty() {
        println!("No todos found.");
        return Ok(());
    }

    let to_remove: Vec<usize> = if all {
        todos.iter().map(|(i, _)| *i).collect()
    } else if query.is_empty() {
        let options: Vec<_> = todos.iter().map(|(_, text)| text.clone()).collect();
        let choices = MultiSelect::new("Select todo(s) to remove:", options).prompt();
        match choices {
            Ok(selected) => selected
                .into_iter()
                .filter_map(|sel| todos.iter().find(|(_, text)| *text == sel).map(|(i, _)| *i))
                .collect(),
            Err(_) => vec![],
        }
    } else {
        let mut scored: Vec<_> = todos
            .iter()
            .filter_map(|(i, text)| {
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

        if best_matches.len() == 1 {
            vec![best_matches[0].1]
        } else {
            let options: Vec<_> = best_matches
                .iter()
                .map(|(_, _, line)| line.to_string())
                .collect();

            let choices =
                MultiSelect::new("Multiple matches found. Select todo(s) to remove:", options)
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
        }
    };

    if to_remove.is_empty() {
        println!("No todos selected.");
        return Ok(());
    }

    let updated: Vec<_> = content
        .lines()
        .enumerate()
        .filter(|(i, _)| !to_remove.contains(i))
        .map(|(_, line)| line.to_string())
        .collect();

    write(get_todo_file_path()?, updated.join("\n"))?;
    println!("Removed {} todo(s).", to_remove.len());
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
        return live_search(todos);
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

fn live_search(items: Vec<(&str, String)>) -> AnyResult<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;

    let mut query = String::new();
    let matcher = SkimMatcherV2::default();

    'search: loop {
        let (cols, rows) = terminal::size()?;
        let search_line = rows - 1;
        let max_results = (rows - 1).min(items.len() as u16);

        let mut matches: Vec<_> = items
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

        matches.sort_by(|a, b| b.0.cmp(&a.0));

        execute!(stdout, Clear(ClearType::All), Hide)?;

        for (i, (_, indices, item)) in matches.iter().take(max_results as usize).enumerate() {
            let line = search_line - 1 - i as u16;
            execute!(stdout, cursor::MoveTo(0, line))?;
            print!("{}", highlight_indices(item, indices));
        }

        if cols > (19 + 8 + query.width()) as u16 {
            execute!(
                stdout,
                cursor::MoveTo(cols - 19, search_line),
                Print("[Esc/Enter to exit]".dim()),
            )?;
        }
        execute!(stdout, cursor::MoveTo(0, search_line), Show)?;

        print!("Search: {query}");

        stdout.flush()?;

        while let Event::Key(key_event) = read()? {
            if !key_event.is_press() {
                continue;
            }
            match key_event.code {
                KeyCode::Char(c) => {
                    query.push(c);
                    break;
                }
                KeyCode::Backspace if !query.is_empty() => {
                    query.pop();
                    break;
                }
                KeyCode::Enter => {
                    break 'search;
                }
                KeyCode::Esc => {
                    break 'search;
                }
                _ => {}
            }
        }
    }
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
