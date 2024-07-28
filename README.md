# Ferrisume

A JSON Resume implementation in Rust.

## Motivation

I got annoyed with the commands of the [original](https://github.com/jsonresume/resume-cli) CLI tool. 
There were also some design choices I didn't agree with (for example, the init subcommand didn't check if there's a json file already, so I lost my progress).

Another reason was to try out [clap](https://github.com/clap-rs/clap) and see if I could make a "production ready" CLI tool with it.

## Building

This tool requires `libwkhtmltox` to build.

Install the binary, either with a package manager or [manually](https://github.com/wkhtmltopdf/packaging/releases).
