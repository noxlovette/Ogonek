use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;

pub struct QsQuery<T>(pub T);

#[derive(Debug)]
pub struct QsQueryRejection(serde_qs::Error);

impl IntoResponse for QsQueryRejection {
    fn into_response(self) -> Response {
        (
            StatusCode::BAD_REQUEST,
            format!("Failed to deserialize query string: {}", self.0),
        )
            .into_response()
    }
}

impl<T, S> FromRequestParts<S> for QsQuery<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = QsQueryRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query().unwrap_or_default();
        let value = serde_qs::from_str(query).map_err(QsQueryRejection)?;
        Ok(QsQuery(value))
    }
}
