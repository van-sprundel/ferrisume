# Ferrisume

A JSON Resume implementation in Rust.

## About

Ferrisume is a small Rust CLI that implements the JSON Resume approach. It separates resume data from themes so you can edit content and presentation independently.

## Installation

Install from crates.io:

```sh
cargo install ferrisume-cli
```

## Usage

Full command reference and examples can be read at [USAGE.md](USAGE.md).

## Themes

[Read here](THEMES.md) for Theme discovery.

## Motivation

There's an official [resume-cli](https://github.com/jsonresume/resume-cli) that doesn't get maintained anymore. The alternative is resumed, but I got a bit annoyed at how it was implemented (for example, the init subcommand didn't check if there's a json file already, so I lost my progress).

Another reason was to try out [clap](https://github.com/clap-rs/clap) and see if I could make a "production ready" CLI tool with it.

## License

See [LICENSE](LICENSE) for license information.
