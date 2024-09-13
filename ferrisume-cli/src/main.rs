use std::io::Write;
use std::{fs::File, path::Path};

use clap::ArgMatches;
use log::{debug, error, info, warn};

use ferrisume_core::{
    export::pdf::PdfExportOptions, export_to_pdf, generate_html, Resume, ThemeManager,
};

mod args;
mod watch;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let matches = args::args();
    let mut theme_manager = ThemeManager::new();

    if let Some(subcommands) = matches.subcommand() {
        match subcommands {
            ("init", _) => {
                let output_path = "resume.json";
                if Path::new(&output_path).exists() {
                    return Err("resume.json already exists".into());
                }

                let resume = Resume::default();
                let resume = serde_json::to_string(&resume)?;
                File::create_new(output_path)?.write_all(resume.as_bytes())?;

                info!("Initialized a resume.json for you!");
            }
            ("export", export_matches) => {
                let format = export_matches.get_one::<String>("format").unwrap();
                let input = export_matches.get_one::<String>("input").unwrap();
                let mut output_path = export_matches.get_one::<String>("output").unwrap().to_string();

                if let Some(theme) = export_matches.get_one::<String>("theme") {
                    theme_manager.set_theme(theme)?;
                }

                debug!("Input: {input}");
                debug!("Format: {format}");
                debug!("Output path: {output_path}");

                let extension = format!(".{}", format);
                if !output_path.ends_with(&extension) {
                    if output_path.contains('.') {
                        warn!("Invalid output name, perhaps the extension differs from the name?");
                    }

                    output_path.push_str(extension.as_str());
                }

                match &*format.to_ascii_lowercase() {
                    "pdf" => {
                        let html = handle_templating(&theme_manager, input)?;
                        handle_pdf_export(&export_matches, &html)?;
                    }
                    "html" => {
                        let html = handle_templating(&theme_manager, input)?;
                        handle_html_export(&html, &output_path)?;
                    }
                    _ => error!("Unknown format '{format}'",),
                }

                info!("Exported {} successfully", output_path);
            }
            ("watch", _) => {
                watch::watch_command().expect("Couldnt start live view");
            }
            _ => {
                error!("No subcommand was used. Use --help for usage information.");
            }
        }
    }

    Ok(())
}

fn handle_templating(
    theme_manager: &ThemeManager,
    input_file: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(input_file)?;
    let input: Resume = serde_json::from_str(&input)?;

    let html = generate_html(theme_manager, &input)?;

    info!("Converted JSON to HTML successfully");
    Ok(html)
}

fn handle_pdf_export(matches: &ArgMatches, html: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = matches.get_one::<String>("output").unwrap();

    let options = PdfExportOptions {
        paper_size: "A4".to_owned(),
        margins: vec!["10".to_owned(); 4],
    };

    export_to_pdf(html, output_path, &options)?;

    Ok(())
}

fn handle_html_export(html: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    File::create(output_path)?.write_all(html.as_bytes())?;

    Ok(())
}
