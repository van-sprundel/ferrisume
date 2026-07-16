# Ferrisume

A JSON Resume implementation in Rust.

## Goal
The goal of a JSON resume is to be able have a data-oriented resume, this makes it easy to edit both the data and theme individually. No more fighting Word!

## Motivation

There's an official [resume-cli](https://github.com/jsonresume/resume-cli) that doesn't get maintained anymore. The alternative is resumed, but I got a bit annoyed at how it was implemented (for example, the init subcommand didn't check if there's a json file already, so I lost my progress).

Another reason was to try out [clap](https://github.com/clap-rs/clap) and see if I could make a "production ready" CLI tool with it.

## Installation

You can download the CLI from using binstall:
```sh
cargo binstall ferrisume-cli
```

## Usage

```
Usage: ferrisume [COMMAND]

Commands:
  init         Initialize a resume.json file
  themes       List all available themes
  version      Display version information
  update       Update ferrisume to the latest release
  validate     Validate a resume file against the JSON Resume schema
  completions  Generate shell completions
  theme        Theme management commands
  watch        Edit your resume in a live view
  export       Export locally to .html or .pdf
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Self-updating

Ferrisume can update itself to the latest GitHub release:

```sh
ferrisume update          # download and install the latest version
ferrisume update --check  # only check whether a newer version exists
```

Other commands print a hint when a newer version is available. The check runs
at most once a day and caches its result; set `FERRISUME_NO_UPDATE_CHECK=1` to
disable it entirely.

## Validation

```sh
ferrisume validate            # validates resume.json
ferrisume validate -i cv.json
```

Checks the file against the official [JSON Resume schema](https://jsonresume.org/schema/)
and reports every violation with its location. All fields are optional — a
resume without a phone number (or without most things) is valid and renders fine.

## Shell completions

```sh
ferrisume completions fish > ~/.config/fish/completions/ferrisume.fish
ferrisume completions bash > ~/.local/share/bash-completion/completions/ferrisume
ferrisume completions zsh  > ~/.zfunc/_ferrisume
```

## Theme Discovery

Ferrisume discovers themes from (in order):

- `./themes` in the current directory
- XDG data dir: `${XDG_DATA_HOME:-$HOME/.local/share}/ferrisume/themes`
- System dir (non-Windows): `/usr/share/ferrisume/themes`
- macOS: `~/Library/Application Support/com.ferrisume.ferrisume/themes`
- Windows: `%APPDATA%\ferrisume\ferrisume\data\themes`
- Next to the executable: `<exe-dir>/themes`

Quick setup on Linux (XDG):

```bash
mkdir -p "${XDG_DATA_HOME:-$HOME/.local/share}/ferrisume/themes/<your-theme>"
# Place `config.toml` and `templates/` inside that directory
```

Verify discovery:

```bash
RUST_LOG=info ferrisume themes
```

Notes:

- If no theme named `default` is found in the filesystem, the embedded default
is extracted to a temporary directory (you may see a `/tmp/.../default` path in
the list).
- You can always reference a theme by path:
`ferrisume export -t /path/to/your/theme`.
