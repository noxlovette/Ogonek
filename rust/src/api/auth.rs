use crate::auth::{ AuthBody, AuthPayload };
use axum::extract::Json;
use axum::extract::State;
use axum::response::Response;
use serde::{ Deserialize, Serialize };
use surrealdb::opt::auth::Record;
use crate::db::error::DbError;
use crate::db::{ AppState, NAMESPACE, DATABASE };

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordUser {
    name: String,
    username: String,
    pass: String,
    email: String,
    id: String,
}

#[derive(Serialize, Deserialize)]
struct Credentials<'a> {
    username: &'a str,
    pass: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SignUpPayload {
    pub name: String,
    pub email: String,
    pub pass: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
struct Params<'a> {
    name: &'a str,
    pass: &'a str,
    email: &'a str,
    username: &'a str,
}

pub async fn authorize(
    State(state): State<AppState>,
    Json(payload): Json<AuthPayload>
    // TODO adjust the error types to accomodate for 'not found disconnected blabla and other db errors'
) -> Result<Response, DbError> {
    tracing::info!("Attempting sign-in for user: {:?}", payload.username);

    let db = &state.db;

    let jwt = db.signin(Record {
        access: "account",
        namespace: &*NAMESPACE,
        database: &*DATABASE,
        params: Credentials {
            username: &payload.username,
            pass: &payload.pass,
        },
    }).await?;

    let token = jwt.as_insecure_token();

    let result: Vec<AuthBody> = db.query("SELECT * FROM user WHERE id = $auth.id").await?.take(0)?;

    let user = result.into_iter().next();

    if let Some(user) = user {
        Ok(user.into_response(token.to_string()))
    } else {
        Err(DbError::Db)
    }
}

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignUpPayload>
) -> Result<Response, DbError> {
    tracing::info!("Creating user: {:?}", payload);

    let db = &state.db;
    let name = payload.name;
    let pass = payload.pass;
    let email = payload.email;
    let username = payload.username;

    let jwt = db.signup(Record {
        access: "account",
        namespace: &*NAMESPACE,
        database: &*DATABASE,
        params: Params {
            name: &name,
            pass: &pass,
            username: &username,
            email: &email,
        },
    }).await?;

    let token = jwt.as_insecure_token();
    let result: Vec<AuthBody> = db.query("SELECT * FROM user WHERE id = $auth.id").await?.take(0)?;
    let user = result.into_iter().next();

    tracing::info!("fetched user: {:?}", user);

    // If user was created successfully, return it with the token in headers
    if let Some(user) = user {
        Ok(user.into_response(token.to_string()))
    } else {
        Err(DbError::Db)
    }
}
