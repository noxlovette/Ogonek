use axum::{
    extract::{Path, Query, State},
    http::Response,
};
use reqwest::{StatusCode, header};

use crate::{
    api::error::APIError,
    Claims,
    openapi::FILE_TAG,
    AppState,
    services::generate_pdf,
};
use ogonek_types::{
    ModelType, PDFQuery, PDFType, PaginatedResponse, PaginatedTasks, TaskPaginationParams,
    TaskSmall, TaskWithFilesResponse,
};

/// Generate the PDF for the requested resource
#[utoipa::path(
    post,
    path = "/{id}", tag = FILE_TAG,
    params(
        ("id" = String, Path, description = "Resource ID. Task by default")
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
) -> Result<Response, APIError> {
    match query.pdf_type {
        None => Ok(Response::new("No PDF Requested")),
        Some(PDFType::Task) => {
            let pdf_bytes = generate_pdf().await?;
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/pdf")
                .header(
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"report.pdf\"",
                )
                .body(pdf_bytes.into())?)
        }
        Some(PDFType::Lesson) => Ok(Response::default()),
    }
}
