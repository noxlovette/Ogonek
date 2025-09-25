use crate::error::AppError;
use axum::{
    http::{StatusCode, header},
    response::Response,
};
use comrak::Options;
use reqwest::multipart;
use serde::Deserialize;
use std::io::Write;
use std::{fs, io::Cursor};
use zip::ZipWriter;
#[derive(Deserialize)]
pub struct PdfGenerationRequest {
    pub html_content: String,
    pub css_content: Option<String>,
}

pub async fn generate_report_zip() -> Result<Response, AppError> {
    let css = fs::read_to_string("./mdconfig.css")?;

    let raw = fs::read_to_string("./input.md")?;

    let mut options = Options::default();
    options.extension.table = true;

    let html = comrak::markdown_to_html(&raw, &options);

    // 1. Préparer le multipart pour Gotenberg
    let mut form = multipart::Form::new();

    form = form.part(
        "files",
        multipart::Part::text(html)
            .file_name("index.html")
            .mime_str("text/html")?,
    );

    form = form.part(
        "files",
        multipart::Part::text(css)
            .file_name("style.css")
            .mime_str("text/css")?,
    );

    // 2. Appeler Gotenberg
    let client = reqwest::Client::new();
    let gotenberg_response = client
        .post(&format!(
            "{}/forms/chromium/convert/html",
            "http://localhost:3002"
        ))
        .multipart(form)
        .send()
        .await?;

    let status = gotenberg_response.status();

    if !status.is_success() {
        let error_body = gotenberg_response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error response".to_string());
        tracing::error!("Gotenberg failed with status {}: {}", status, error_body);

        return Err(AppError::Internal(format!("{error_body}")));
    } else {
        let pdf_bytes = gotenberg_response.bytes().await?;

        let zip_data = create_zip_archive(pdf_bytes).await?;

        Ok(Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/zip")
            .header(
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"report.zip\"",
            )
            .body(zip_data.into())?)
    }
}

async fn create_zip_archive(pdf_bytes: bytes::Bytes) -> Result<Vec<u8>, AppError> {
    let buffer = Vec::new();
    let cursor = Cursor::new(buffer);
    let mut zip = ZipWriter::new(cursor);

    zip.start_file("report.pdf", zip::write::SimpleFileOptions::default())?;
    zip.write_all(&pdf_bytes)?;

    let cursor = zip.finish()?;
    Ok(cursor.into_inner())
}
