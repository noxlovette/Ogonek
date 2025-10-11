use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use ogonek_db::core::{lesson, task};

use crate::{AppState, Claims, api::error::APIError, openapi::FILE_TAG, services::generate_pdf};
use ogonek_types::{PDFQuery, PDFType};

/// Generates or retrieves a PDF for the specified resource
///
/// Creates a PDF for tasks or lessons, caches it in S3, and returns a presigned URL.
#[utoipa::path(
    post,
    path = "/pdf/{id}", tag = FILE_TAG,
    params(
        ("id" = String, Path, description = "Resource ID"),
        ("pdfType" = String, Query, description = "Resource type")
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
) -> Result<impl IntoResponse, APIError> {
    let pdf_type = query
        .pdf_type
        .ok_or_else(|| APIError::BadRequest("No PDF type specified".into()))?;

    let (raw_data, filename, s3_key) = match pdf_type {
        PDFType::Task => {
            let data = task::read_for_pdf(&state.db, &id, &claims.sub).await?;
            let filename = format!("task_{}.pdf", sanitize_filename(&data.title));
            let s3_key = format!("exports/{}/tasks/{}.pdf", claims.sub, id);
            (data, filename, s3_key)
        }
        PDFType::Lesson => {
            let data = lesson::read_for_pdf(&state.db, &id, &claims.sub).await?;
            let filename = format!("lesson_{}.pdf", sanitize_filename(&data.title));
            let s3_key = format!("exports/{}/lessons/{}.pdf", claims.sub, id);
            (data, filename, s3_key)
        }
    };

    // Check si le PDF existe déjà sur S3
    let presigned_url = match state.s3.object_exists(&s3_key).await {
        Ok(true) => {
            // Déjà là, just presign
            tracing::info!(s3_key = %s3_key, "PDF exists, generating presigned URL");
            state.s3.get_presigned_url(s3_key, filename).await?
        }
        _ => {
            // Pas là ou erreur, génère + upload
            tracing::info!(s3_key = %s3_key, "Generating new PDF");
            let pdf_bytes = generate_pdf(raw_data).await?;

            state
                .s3
                .upload_object(&s3_key, pdf_bytes.into(), Some("application/pdf"))
                .await?;

            // Presign
            state.s3.get_presigned_url(s3_key, filename).await?
        }
    };

    // Return JSON avec l'URL au lieu du blob
    Ok(Json(serde_json::json!({
        "url": presigned_url
    })))
}

/// Sanitizes a filename by replacing invalid characters
///
/// Converts special characters to underscores for safe file naming.
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
