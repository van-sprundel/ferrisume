use headless_chrome::{types::PrintToPdfOptions, Browser, LaunchOptionsBuilder};
use std::fmt::Debug;
use std::io;
use tempfile;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PdfExportError {
    #[error("Invalid margin value: {0}")]
    InvalidMarginValue(#[from] std::num::ParseFloatError),
    #[error("Headless Chrome error: {0}")]
    HeadlessChromeError(String),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Failed to create temporary directory: {0}")]
    TempDirError(#[from] tempfile::PersistError),
    #[error("Failed to navigate to page: {0}")]
    NavigationError(String),
    #[error("Failed to print to PDF: {0}")]
    PrintToPdfError(String),
}

pub fn export_to_pdf(html_content: &str, output_path: &str) -> Result<(), PdfExportError> {
    let options = PrintToPdfOptions {
        prefer_css_page_size: Some(true),
        ..Default::default()
    };

    let pdf_data = print_to_pdf(html_content, options)?;
    std::fs::write(output_path, pdf_data)?;
    Ok(())
}

fn print_to_pdf(html_content: &str, options: PrintToPdfOptions) -> Result<Vec<u8>, PdfExportError> {
    let launch_options = LaunchOptionsBuilder::default()
        .headless(true)
        .build()
        .map_err(|e| {
            PdfExportError::HeadlessChromeError(format!("Failed to build launch options: {}", e))
        })?;

    let browser = Browser::new(launch_options).map_err(|e| {
        PdfExportError::HeadlessChromeError(format!("Failed to launch browser: {}", e))
    })?;

    let tab = browser.new_tab().map_err(|e| {
        PdfExportError::HeadlessChromeError(format!("Failed to create new tab: {}", e))
    })?;

    // Create temporary HTML file
    let temp_dir = tempfile::TempDir::new()?;
    let temp_file_path = temp_dir.path().join("temp.html");
    std::fs::write(&temp_file_path, html_content)?;

    // Navigate to the temporary HTML file
    let file_url = format!("file://{}", temp_file_path.to_string_lossy());
    tab.navigate_to(&file_url)
        .map_err(|e| PdfExportError::NavigationError(e.to_string()))?;

    tab.wait_until_navigated().map_err(|e| {
        PdfExportError::NavigationError(format!("Failed to wait for navigation: {}", e))
    })?;

    tab.print_to_pdf(Some(options))
        .map_err(|e| PdfExportError::PrintToPdfError(e.to_string()))
}
