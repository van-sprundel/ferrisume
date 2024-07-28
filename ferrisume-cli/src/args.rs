use clap::{App, Arg, ArgMatches, SubCommand};

pub fn args() -> ArgMatches<'static> {
    App::new("ferrisume")
        .version("0.0")
        .author("Ramon van Sprundel <ramonvansprundel@gmail.com>")
        .about("A resume generator CLI tool")
        .subcommand(SubCommand::with_name("init").about("Initialize a resume.json file"))
        .subcommand(SubCommand::with_name("watch").about("Edit your resume in a live view"))
        .subcommand(
            SubCommand::with_name("export")
                .about("Export locally to .html or .pdf")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .value_name("INPUT")
                        .help("Specify input file (defaults to resume.json)")
                        .takes_value(true)
                        .default_value("resume.json"),
                )
                .arg(
                    Arg::with_name("format")
                        .short("f")
                        .long("format")
                        .value_name("FORMAT")
                        .help("Specify output format (pdf or html)")
                        .takes_value(true)
                        .default_value("pdf"),
                )
                .arg(
                    Arg::with_name("theme")
                        .short("t")
                        .long("theme")
                        .value_name("THEME")
                        .help("Specify theme used by `export` or specify a path starting with .")
                        .takes_value(true)
                        .default_value("default-theme"),
                )
                .arg(
                    Arg::with_name("output")
                        .help("Output file name")
                        .required(false)
                        .index(1)
                        .default_value("resume"),
                ),
        )
        .get_matches()
}
