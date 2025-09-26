use crate::error::AppError;
use axum::{
    http::{StatusCode, header},
    response::Response,
};
use comrak::{Options, markdown_to_html};
use reqwest::multipart;
use std::{
    fs,
    io::{Cursor, Write},
};
use zip::ZipWriter;

fn render_markdown_page(markdown: &str, css_path: &str) -> String {
    let mut options = Options::default();
    options.extension.table = true;
    let content = markdown_to_html(markdown, &options);

    let topic = "Flying Whales";
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">

    <link rel="stylesheet" href="{}">
    <title>Document</title>
</head>
<body>
<h1>{}</h1>
    <article>
        {}
    </article>
</body>
</html>"#,
        css_path, topic, content
    )
}
pub async fn generate_report_zip() -> Result<Response, AppError> {
    let css = fs::read_to_string("./mdconfig.css")?;

    let raw = fs::read_to_string("./input.md")?;

    let html = render_markdown_page(&raw, "mdconfig.css");

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
            .file_name("mdconfig.css")
            .mime_str("text/css")?,
    );

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
