use std::fmt::Debug;
use std::io;
use std::io::Write;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use wkhtmltopdf::{Orientation, PageSize, PdfApplication, Size};

#[derive(Error, Debug)]
pub enum PdfExportError {
    #[error("Invalid paper size {0}")]
    InvalidPaperSize(String),
    #[error("Invalid margin definition '{0}'")]
    InvalidMarginDefinition(String),
    #[error("Invalid margin value: {0}")]
    InvalidMarginValue(std::num::ParseFloatError),
    #[error("Headless chrome error: {0}")]
    HeadlessChromeError(#[from] anyhow::Error),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("wkhtmltopdf error: {0}")]
    WkHtmlToPdfError(#[from] wkhtmltopdf::Error),
}

#[derive(Serialize, Deserialize)]
pub struct PdfExportOptions {
    pub paper_size: String,
    pub margins: Vec<String>,
}

pub fn export_to_pdf(
    html_content: &str,
    output_path: &str,
    options: &PdfExportOptions,
) -> Result<(), PdfExportError> {
    let pdf_app = PdfApplication::new()?;

    let mut pdf = &mut pdf_app.builder();

    pdf = pdf
        .orientation(Orientation::Portrait)
        .margin(Size::Inches(options.margins[0].parse().unwrap_or(10)))
        .title("Resume");

    match options.paper_size.to_lowercase().as_str() {
        "a4" => pdf = pdf.page_size(PageSize::A4),
        "letter" => pdf = pdf.page_size(PageSize::Letter),
        _ => return Err(PdfExportError::InvalidPaperSize(options.paper_size.clone())),
    }

    pdf.build_from_html(html_content)?.save(&output_path)?;

    Ok(())
}
