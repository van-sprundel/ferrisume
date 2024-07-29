# Ferrisume

A JSON Resume implementation in Rust.

## Goal
The goal of a JSON resume is to be able to store a resume in data and edit the data/theme autonomously and on-the-fly. No more battling Word anymore!

## Motivation

There's an official [resume-cli](https://github.com/jsonresume/resume-cli) that doesn't get maintained anymore. The alternative is resumed, but I got a bit annoyed at how it was implemented (for example, the init subcommand didn't check if there's a json file already, so I lost my progress). 

Another reason was to try out [clap](https://github.com/clap-rs/clap) and see if I could make a "production ready" CLI tool with it.

## Installation

You can download the CLI from cargo:
```sh
cargo install ferrisume-cli
```

## Usage

```
USAGE:
    ferrisume [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    export    Export locally to .html or .pdf
    help      Prints this message or the help of the given subcommand(s)
    init      Initialize a resume.json file
    watch     Edit your resume in a live view
```

## Building

This tool requires `libwkhtmltox` to build.

Install the binary, either with a package manager or [manually](https://github.com/wkhtmltopdf/packaging/releases).
