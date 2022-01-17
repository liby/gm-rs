# Git multiple user config manager

> This project was initialized from [gum](https://github.com/gauseen/gum)

## Installation

### Download binary

Download from [release page](https://github.com/liby/gm-rs/releases/latest), and extract to the directory in PATH.

## Usage

### List all the user config group

```sh
$ gum list

╔═════════════════════════════════════╦═══════════╦═════════════════════════════╗
║                Scope                ║    Name   ║            Email            ║
╠═════════════════════════════════════╬═══════════╬═════════════════════════════╣
║ global                              ║ admin     ║ admin@email.com             ║
╠═════════════════════════════════════╬═══════════╬═════════════════════════════╣
║ includeif "gitdir:~/code/personal/" ║ personal  ║ personal@email.com          ║
╠═════════════════════════════════════╬═══════════╬═════════════════════════════╣
║ includeif "gitdir:~/code/company/"  ║ company   ║ company@email.com           ║
╚═════════════════════════════════════╩═══════════╩═════════════════════════════╝
```

### Set one group for user config
```sh
$ gum use personal

Currently used name=personal email=company@email.com
```

## More info

```sh
USAGE:
    gum <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    delete    Delete one group
    help      Print this message or the help of the given subcommand(s)
    list      List all the user config group
    set       Set one group for user config
    use       Use one group name for user config
```
