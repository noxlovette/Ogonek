use axum::{
    body::Body,
    http::{Request, StatusCode},
};

mod common;
use common::*;
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

#[sqlx::test]
async fn test_auth_flow_end_to_end(db: PgPool) {
    let app_state = create_test_app_state(db).await;
    let app = create_test_app(app_state).await;

    // Test signup
    let signup_payload = json!({
        "name": "Test User",
        "username": "testuser",
        "email": "test@example.com",
        "pass": "securepassword123",
        "role": "student"
    });

    let signup_request = Request::builder()
        .method("POST")
        .uri("/auth/signup")
        .header("content-type", "application/json")
        .header(
            "x-api-key",
            std::env::var("API_KEY").unwrap_or("test-key".to_string()),
        )
        .body(Body::from(signup_payload.to_string()))
        .unwrap();

    let signup_response = app.clone().oneshot(signup_request).await.unwrap();
    assert_eq!(signup_response.status(), StatusCode::OK);
    let body = signup_response.into_body();
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();

    // Test signin
    let signin_payload = json!({
        "username": "testuser",
        "pass": "securepassword123"
    });

    let signin_request = Request::builder()
        .method("POST")
        .uri("/auth/signin")
        .header("content-type", "application/json")
        .header(
            "x-api-key",
            std::env::var("API_KEY").unwrap_or("test-key".to_string()),
        )
        .body(Body::from(signin_payload.to_string()))
        .unwrap();

    let signin_response = app.clone().oneshot(signin_request).await.unwrap();
    assert_eq!(signin_response.status(), StatusCode::OK);
    let body = signin_response.into_body();
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    let signin_result: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

    let access_token = signin_result["accessToken"]["token"].as_str().unwrap();
    let refresh_token = signin_result["refreshToken"]["token"].as_str().unwrap();

    // Test protected endpoint with access token
    let protected_request = Request::builder()
        .method("GET")
        .uri("/user")
        .header("authorization", format!("Bearer {access_token}"))
        .header(
            "x-api-key",
            std::env::var("API_KEY").unwrap_or("test-key".to_string()),
        )
        .body(Body::empty())
        .unwrap();

    let protected_response = app.clone().oneshot(protected_request).await.unwrap();
    assert_eq!(protected_response.status(), StatusCode::OK);

    // Test token refresh
    let refresh_payload = json!({
        "refreshToken": refresh_token
    });

    let refresh_request = Request::builder()
        .method("POST")
        .uri("/auth/refresh")
        .header("content-type", "application/json")
        .header(
            "x-api-key",
            std::env::var("API_KEY").unwrap_or("test-key".to_string()),
        )
        .body(Body::from(refresh_payload.to_string()))
        .unwrap();

    let refresh_response = app.clone().oneshot(refresh_request).await.unwrap();
    assert_eq!(refresh_response.status(), StatusCode::OK);

    // Test invalid credentials
    let invalid_signin = json!({
        "username": "testuser",
        "pass": "wrongpassword"
    });

    let invalid_request = Request::builder()
        .method("POST")
        .uri("/auth/signin")
        .header("content-type", "application/json")
        .header(
            "x-api-key",
            std::env::var("API_KEY").unwrap_or("test-key".to_string()),
        )
        .body(Body::from(invalid_signin.to_string()))
        .unwrap();

    let invalid_response = app.clone().oneshot(invalid_request).await.unwrap();
    assert_eq!(invalid_response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn test_signup_validation_errors(db: PgPool) {
    let app_state = create_test_app_state(db).await;
    let app = create_test_app(app_state).await;

    // Test duplicate username
    let user1 = json!({
        "name": "User One",
        "username": "duplicate",
        "email": "user1@example.com",
        "pass": "password123",
        "role": "student"
    });

    let request1 = Request::builder()
        .method("POST")
        .uri("/auth/signup")
        .header("content-type", "application/json")
        .header(
            "x-api-key",
            std::env::var("API_KEY").unwrap_or("test-key".to_string()),
        )
        .body(Body::from(user1.to_string()))
        .unwrap();

    let response1 = app.clone().oneshot(request1).await.unwrap();
    assert_eq!(response1.status(), StatusCode::OK);

    // Try to create another user with same username
    let user2 = json!({
        "name": "User Two",
        "username": "duplicate",
        "email": "user2@example.com",
        "pass": "password123",
        "role": "student"
    });

    let request2 = Request::builder()
        .method("POST")
        .uri("/auth/signup")
        .header("content-type", "application/json")
        .header(
            "x-api-key",
            std::env::var("API_KEY").unwrap_or("test-key".to_string()),
        )
        .body(Body::from(user2.to_string()))
        .unwrap();

    let response2 = app.clone().oneshot(request2).await.unwrap();
    assert_eq!(response2.status(), StatusCode::CONFLICT);

    // Test invalid email format
    let invalid_user = json!({
        "name": "Invalid User",
        "username": "invaliduser",
        "email": "not-an-email",
        "pass": "password123",
        "role": "student"
    });

    let invalid_request = Request::builder()
        .method("POST")
        .uri("/auth/signup")
        .header("content-type", "application/json")
        .header(
            "x-api-key",
            std::env::var("API_KEY").unwrap_or("test-key".to_string()),
        )
        .body(Body::from(invalid_user.to_string()))
        .unwrap();

    let invalid_response = app.clone().oneshot(invalid_request).await.unwrap();
    assert_eq!(invalid_response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn test_api_key_middleware(db: PgPool) {
    let app_state = create_test_app_state(db).await;
    let app = create_test_app(app_state).await;

    // Test without API key
    let request = Request::builder()
        .method("POST")
        .uri("/auth/signup")
        .header("content-type", "application/json")
        .body(Body::from("{}"))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // Test with wrong API key
    let request = Request::builder()
        .method("POST")
        .uri("/auth/signup")
        .header("content-type", "application/json")
        .header("x-api-key", "wrong-key")
        .body(Body::from("{}"))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
