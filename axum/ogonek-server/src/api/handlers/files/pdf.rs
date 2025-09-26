use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::Response,
};
use bytes::Bytes;
use ogonek_db::core::{lesson, task};
use reqwest::{StatusCode, header};

use crate::{AppState, Claims, api::error::APIError, openapi::FILE_TAG, services::generate_pdf};
use ogonek_types::{PDFQuery, PDFType};

#[axum::debug_handler]
/// Generate the PDF for the requested resource
#[utoipa::path(
    post,
    path = "/pdf/{id}", tag = FILE_TAG,
    params(
        ("id" = String, Path, description = "Resource ID")
    ),
    responses(
        (status = 200, description = "Requested PDF generated"),
        (status = 404, description = "Resource not found"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn get_pdf(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Query(query): Query<PDFQuery>,
    claims: Claims,
) -> Result<Response<Body>, APIError> {
    let pdf_type = query
        .pdf_type
        .ok_or_else(|| APIError::BadRequest("No PDF type specified".into()))?;

    let (raw_data, filename) = match pdf_type {
        PDFType::Task => {
            let data = task::read_for_pdf(&state.db, &id, &claims.sub).await?;
            let filename = format!("task_{}.pdf", sanitize_filename(&data.title));
            (data, filename)
        }
        PDFType::Lesson => {
            let data = lesson::read_for_pdf(&state.db, &id, &claims.sub).await?;
            let filename = format!("lesson_{}.pdf", sanitize_filename(&data.title));
            (data, filename)
        }
    };

    let pdf_bytes = generate_pdf(raw_data).await?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/pdf")
        .header(
            header::CONTENT_DISPOSITION,
            format!(r#"attachment; filename="{}""#, filename),
        )
        .body(pdf_bytes.into())?)
}

// Helper function pour nettoyer le nom de fichier
fn sanitize_filename(title: &str) -> String {
    title
        .chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => c,
            ' ' => '_',
            _ => '_',
        })
        .collect::<String>()
        .trim_matches('_')
        .to_string()
}
