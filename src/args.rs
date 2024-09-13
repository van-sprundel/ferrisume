use clap::{Arg, ArgMatches, Command};

pub fn args() -> ArgMatches {
    Command::new("ferrisume")
        .version("0.0")
        .author("Ramon van Sprundel <ramonvansprundel@gmail.com>")
        .about("A resume generator CLI tool")
        .subcommand(Command::new("init").about("Initialize a resume.json file"))
        .subcommand(Command::new("watch").about("Edit your resume in a live view"))
        .subcommand(
            Command::new("export")
                .about("Export locally to .html or .pdf")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("INPUT")
                        .help("Specify input file (defaults to resume.json)")
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
                        .help("Specify theme used by `export` or specify a path starting with .")
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
