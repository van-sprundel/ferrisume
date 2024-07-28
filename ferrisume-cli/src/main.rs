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

    if let Some(theme) = matches.value_of("theme") {
        theme_manager.set_theme(theme)?;
    }

    match matches.subcommand() {
        ("init", Some(_)) => {
            let output_path = "resume.json";
            if Path::new(&output_path).exists() {
                return Err("resume.json already exists".into());
            }

            let resume = Resume::default();
            let resume = serde_json::to_string(&resume)?;
            File::create_new(output_path)?.write(&resume.as_bytes())?;

            info!("Initialized a resume.json for you!");
        }
        ("export", Some(export_matches)) => {
            let format = export_matches.value_of("format").unwrap();
            let input = export_matches.value_of("input").unwrap();
            let mut output_path = export_matches.value_of("output").unwrap().to_string();

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
                    handle_pdf_export(&matches, &html)?;
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
    let output_path = matches.value_of("output").unwrap_or("resume.pdf");

    let options = PdfExportOptions {
        paper_size: matches.value_of("paper-size").unwrap_or("A4").to_string(),
        margins: matches
            .values_of("margin")
            .map(|v| v.map(String::from).collect())
            .unwrap_or_else(|| vec!["10".to_string(); 4]),
    };

    export_to_pdf(html, output_path, &options)?;

    Ok(())
}

fn handle_html_export(html: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    File::create(output_path)?.write_all(html.as_bytes())?;

    Ok(())
}
