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

- [x] Windows support

- [ ] Test on MacOS

