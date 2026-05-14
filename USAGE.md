# Usage

This document describes ferrisume's commands, options, and examples.

## Commands

```sh
Usage: ferrisume [COMMAND]

Commands:
  init     Initialize a resume.json file
  themes   List all available themes
  version  Display version information
  theme    Theme management commands
  watch    Edit your resume in a live view
  export   Export locally to .html or .pdf
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Export subcommand

```sh
Usage: ferrisume export [OPTIONS] [OUTPUT]

Arguments:
  [OUTPUT]  Output file name [default: resume.pdf]

Options:
  -i, --input <INPUT>    Specify input file [default: resume.json]
  -f, --format <FORMAT>  Specify output format (pdf or html) [default: pdf]
  -t <THEME>             Specify theme to use (name or path) [default: default]
```

## Examples

Create a PDF using the default input and format:

```sh
ferrisume export
```

Specify an input file and format explicitly:

```sh
ferrisume export -i resume.json -f pdf
```
