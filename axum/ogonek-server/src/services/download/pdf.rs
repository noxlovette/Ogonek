use crate::{AppError, services::download::markdown};
use ogonek_types::PDFData;
use reqwest::multipart;

pub async fn generate_pdf(raw: PDFData) -> Result<axum::body::Bytes, AppError> {
    const CSS_CONFIG: &str = include_str!("mdconfig.css");
    const GOTENBERG_ADDRESS: &str = "http://gotenberg:3000";
    let css = CSS_CONFIG;

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
        .post(format!("{}/forms/chromium/convert/html", GOTENBERG_ADDRESS))
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

        Err(AppError::Internal(error_body.to_string()))
    } else {
        Ok(gotenberg_response.bytes().await?)
    }
}
