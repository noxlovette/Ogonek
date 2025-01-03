use crate::db::error::DbError;
use crate::db::{AppState, DATABASE, NAMESPACE};
use crate::models::users::{AuthPayload, Credentials, SignUpCredentials, SignUpPayload, User};
use axum::extract::Json;
use axum::extract::State;
use axum::response::Response;
use surrealdb::opt::auth::Record;

pub async fn authorize(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>, // TODO adjust the error types to accomodate for 'not found disconnected blabla and other db errors'
) -> Result<Response, DbError> {
    tracing::info!("Attempting sign-in for user: {:?}", payload.username);

    let db = &state.db;

    let jwt = db
        .signin(Record {
            access: "user",
            namespace: &*NAMESPACE,
            database: &*DATABASE,
            params: Credentials {
                username: &payload.username,
                pass: &payload.pass,
            },
        })
        .await?;

    let token = jwt.as_insecure_token();

    tracing::info!("token: {:?}", token);

    let result: Vec<User> = db
        .query("SELECT * FROM user WHERE id = $auth.id")
        .await?
        .take(0)?;

    tracing::info!("result: {:?}", result);

    let user = result.into_iter().next();

    if let Some(user) = user {
        Ok(user.into_response(token.to_string()))
    } else {
        Err(DbError::Db)
    }
}

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignUpPayload>,
) -> Result<Response, DbError> {
    tracing::info!("Creating user: {:?}", payload);

    let db = &state.db;
    let name = payload.name;
    let pass = payload.pass;
    let email = payload.email;
    let username = payload.username;
    let role = payload.role;

    let jwt = db
        .signup(Record {
            access: "user",
            namespace: &*NAMESPACE,
            database: &*DATABASE,
            params: SignUpCredentials {
                name: &name,
                pass: &pass,
                username: &username,
                email: &email,
                role: &role,
            },
        })
        .await?;

    let token = jwt.as_insecure_token();
    let result: Vec<User> = db
        .query("SELECT * FROM user WHERE id = $auth.id")
        .await?
        .take(0)?;
    let user = result.into_iter().next();

    tracing::info!("fetched user: {:?}", user);

    // If user was created successfully, return it with the token in headers
    if let Some(user) = user {
        Ok(user.into_response(token.to_string()))
    } else {
        Err(DbError::Db)
    }
}
