use crate::{api::routes::root, schema::AppState};
#[cfg(test)]
use axum::{body::Body, http::Request};
use reqwest::StatusCode;
use tower::ServiceExt;

#[tokio::test]
async fn test_user_creation() {
    let state = AppState::test().await.unwrap();
    let app = root(state.clone(), "localhost".to_string()).unwrap();
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/signup")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"name": "test_user", "email": "test@example.com", "pass": "passwowrd123", "role": "teacher", "username": "test"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let user = sqlx::query_scalar!(
        r#"
        SELECT name FROM "user" WHERE email = $1
        "#,
        "test@example.com"
    )
    .fetch_one(&state.db)
    .await
    .unwrap();

    assert_eq!(user, "test_user");
}
