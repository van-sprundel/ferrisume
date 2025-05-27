use clap::{Arg, ArgMatches, Command};

pub fn args() -> ArgMatches {
    Command::new("ferrisume")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Ramon van Sprundel <ramonvansprundel@gmail.com>")
        .about("A resume generator CLI tool")
        .subcommand(Command::new("init").about("Initialize a resume.json file"))
        .subcommand(Command::new("themes").about("List all available themes"))
        .subcommand(Command::new("version").about("Display version information"))
        .subcommand(
            Command::new("theme")
                .about("Theme management commands")
                .subcommand(
                    Command::new("create")
                        .about("Create a new custom theme")
                        .arg(
                            Arg::new("name")
                                .help("Name of the new theme")
                                .required(true)
                                .index(1),
                        )
                        .arg(
                            Arg::new("base")
                                .short('b')
                                .long("base")
                                .help("Base theme to use as a template")
                                .default_value("default")
                                .required(false),
                        ),
                ),
        )
        .subcommand(
            Command::new("watch")
                .about("Edit your resume in a live view")
                .arg(
                    Arg::new("theme")
                        .short('t')
                        .long("theme")
                        .help("Specify theme to use (name or path)")
                        .required(false)
                        .default_value("default"),
                )
                .arg(
                    Arg::new("http-port")
                        .long("http-port")
                        .help("HTTP port for the preview server (default: 8000)")
                        .required(false)
                        .value_parser(clap::value_parser!(u16))
                        .default_value("8000"),
                )
                .arg(
                    Arg::new("ws-port")
                        .long("ws-port")
                        .help("WebSocket port for live reloading (default: 9000)")
                        .required(false)
                        .value_parser(clap::value_parser!(u16))
                        .default_value("9000"),
                ),
        )
        .subcommand(
            Command::new("export")
                .about("Export locally to .html or .pdf")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("INPUT")
                        .help("Specify input file")
                        .num_args(1)
                        .required(false)
                        .default_value("resume.json"),
                )
                .arg(
                    Arg::new("format")
                        .short('f')
                        .long("format")
                        .value_name("FORMAT")
                        .help("Specify output format (pdf or html)")
                        .num_args(1)
                        .required(false)
                        .default_value("pdf"),
                )
                .arg(
                    Arg::new("theme")
                        .short('t')
                        .value_name("THEME")
                        .help("Specify theme to use (name or path)")
                        .num_args(1)
                        .required(false)
                        .default_value("default"),
                )
                .arg(
                    Arg::new("output")
                        .value_name("OUTPUT")
                        .help("Output file name")
                        .required(false)
                        .index(1)
                        .default_value("resume.pdf"),
                ),
        )
        .get_matches()
}
