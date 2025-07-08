# todo

An easy way to manage your todos

## Installation

```bash
cargo install to-dos
```

## Usage

```bash
$ cd any_directory
$ todo
```

A todo file for this directory will be created under `~/.todo/todos/`

For windows, `%USERPROFILE%\.todo\todos\`


```txt
Usage: todo [COMMAND]

Commands:
  sync     Create a hard link to this directory's todo file in the current directory
  unsync   Undo the sync command
  list     List the todos for the current directory
  config   Interactive configuration for the todo cli
  remove   Removes the todo for the current directory. Use todo help remove for other options
  update   Update todo cli
  search   Fuzzily find todos
  check    Fuzzily find todos, check them
  uncheck  Fuzzily find todos, uncheck them
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Configuration

The config file is located at `~/.todo/config.toml`

For windows, `%USERPROFILE%\.todo\config.toml`

### Default config:

```toml
filename = "todo"
extension = ".md" # must include the dot
editor = "$EDITOR" # can be an environment variable or a hardcoded command
```

### Note: 

- On **Windows**, `$EDITOR` isn't set to anything, you can
    
    - Set it globally (`setx EDITOR notepad`)

    - Change the `editor` in the config

## To-dos:

- [x] Publish on crates.io
- [x] Configure editor (defaults to `$EDITOR`)
- [x] Configure file type (defaults to markdown)
- [ ] Sub-commands, args
    - [x] sync/unsync
    - [x] remove
    - [x] config
      - [x] list
    - [x] update
    - [x] search
    - [x] check/done
    - [x] uncheck/undo
- [x] Windows support
- [ ] Test on MacOS

