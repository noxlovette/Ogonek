use crate::error::AppError;
use reqwest::multipart;
use std::fs;
mod markdown;
pub async fn generate_pdf() -> Result<axum::body::Bytes, AppError> {
    let css = fs::read_to_string("./mdconfig.css")?;

    let raw = fs::read_to_string("./input.md")?;

    let html = markdown::render_markdown_page(&raw, "mdconfig.css");

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
        return Ok(gotenberg_response.bytes().await?);
    }
}
