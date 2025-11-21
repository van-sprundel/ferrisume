use core::{export_to_pdf, generate_html, ThemeManager};
use std::io::Write;
use std::{fs::File, path::Path};

use clap::ArgMatches;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use domain::Resume;
use log::{debug, error, info, warn};

mod args;
mod core;
mod domain;
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
                let resume = serde_json::to_string_pretty(&resume)?;
                File::create_new(output_path)?.write_all(resume.as_bytes())?;

                println!("Initialized a resume.json for you!");
            }
            ("themes", _) => {
                let themes = theme_manager.get_all_themes();

                if themes.is_empty() {
                    println!("No themes found");
                    return Ok(());
                }

                println!("\n✨ Available themes ✨\n");

                let mut table = Table::new();
                table
                    .set_header(vec!["NAME", "DESCRIPTION", "AUTHOR", "VERSION", "PATH"])
                    .set_content_arrangement(ContentArrangement::DynamicFullWidth);

                for (name, theme) in themes {
                    let config = &theme.config;
                    table.add_row(vec![
                        Cell::new(name)
                            .fg(Color::Green)
                            .add_attribute(Attribute::Bold),
                        Cell::new(&config.description),
                        Cell::new(&config.author),
                        Cell::new(&config.version).fg(Color::Cyan),
                        Cell::new(theme.path.to_str().unwrap_or("N/A")),
                    ]);
                }

                println!("{table}");
            }
            ("export", export_matches) => {
                let format = export_matches.get_one::<String>("format").unwrap();
                let input = export_matches.get_one::<String>("input").unwrap();
                let mut output_path = export_matches
                    .get_one::<String>("output")
                    .unwrap()
                    .to_string();

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
                        handle_pdf_export(export_matches, &html)?;
                    }
                    "html" => {
                        let html = handle_templating(&theme_manager, input)?;
                        handle_html_export(&html, &output_path)?;
                    }
                    _ => error!("Unknown format '{format}'",),
                }

                println!("Exported {} successfully", output_path);
            }
            ("watch", watch_matches) => {
                let theme = watch_matches.get_one::<String>("theme").unwrap();
                let http_port = watch_matches.get_one::<u16>("http-port").unwrap();
                let ws_port = watch_matches.get_one::<u16>("ws-port").unwrap();

                if let Err(e) = watch::watch_command(theme, *http_port, *ws_port) {
                    error!("Couldn't start live view: {}", e);
                    return Err(format!("Error starting watch: {}", e).into());
                }
            }
            ("theme", theme_matches) => {
                if let Some(theme_command) = theme_matches.subcommand() {
                    match theme_command {
                        ("create", create_matches) => {
                            let name = create_matches.get_one::<String>("name").unwrap();
                            let base = create_matches.get_one::<String>("base").unwrap();
                            create_custom_theme(name, base)?;
                        }
                        _ => {
                            error!("Unknown theme subcommand. Use --help for usage information.");
                        }
                    }
                }
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

    export_to_pdf(html, output_path)?;

    Ok(())
}

fn handle_html_export(html: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    File::create(output_path)?.write_all(html.as_bytes())?;

    Ok(())
}

fn create_custom_theme(name: &str, base: &str) -> Result<(), Box<dyn std::error::Error>> {
    let themes_dir = Path::new("themes");
    if !themes_dir.exists() {
        std::fs::create_dir_all(themes_dir)?;
    }

    let new_theme_dir = themes_dir.join(name);
    if new_theme_dir.exists() {
        return Err(format!("Theme '{}' already exists", name).into());
    }
    std::fs::create_dir_all(&new_theme_dir)?;

    let templates_dir = new_theme_dir.join("templates");
    std::fs::create_dir_all(&templates_dir)?;

    let mut theme_manager = ThemeManager::new();
    theme_manager.set_theme(base)?;
    let base_theme = theme_manager
        .get_current_theme()
        .ok_or_else(|| format!("Base theme '{}' not found", base))?;

    let base_templates_dir = base_theme.path.join("templates");
    if base_templates_dir.exists() {
        copy_dir_contents(&base_templates_dir, &templates_dir)?;
    }

    let config_content = format!(
        r#"name = "{}"
description = "Custom theme based on {}"
author = "Generated by ferrisume"
version = "0.1.0"
"#,
        name, base
    );
    std::fs::write(new_theme_dir.join("config.toml"), config_content)?;

    println!(
        "✅ Created custom theme '{}' in {}",
        name,
        new_theme_dir.display()
    );
    println!("You can now edit the theme files in your IDE and use it with:");
    println!("- ferrisume watch --theme {}", name);
    println!("- ferrisume export --theme {}", name);

    Ok(())
}

fn copy_dir_contents(source: &Path, destination: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = destination.join(path.file_name().unwrap());

        if path.is_dir() {
            std::fs::create_dir_all(&dest_path)?;
            copy_dir_contents(&path, &dest_path)?;
        } else {
            std::fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}
