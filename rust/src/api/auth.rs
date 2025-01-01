use crate::auth::{AuthBody, AuthPayload, AuthError};
use axum::extract::Json;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Record;
use crate::db::error::Error as DbError;
use crate::db::{AppState, NAMESPACE, DATABASE};

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordUser {
    name: String,
    username: String,
    pass: String,
    email: String,
    jwt: String,
}

#[derive(Serialize, Deserialize)]
struct Credentials<'a> {
    username: &'a str,
    pass: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonData {
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

) -> Result<Json<AuthBody>, AuthError> {

    if payload.username.is_empty() || payload.pass.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    tracing::info!("Attempting sign-in for user: {:?}", payload.username);

    let db = &state.db;

    let jwt = db
    .signin(Record {
        access: "account",
        namespace: &*NAMESPACE,
        database: &*DATABASE,
        params: Credentials {
            username: &payload.username,
            pass: &payload.pass,
        },
    })
    .await.map_err(|_| AuthError::WrongCredentials)?;

    let token = jwt.as_insecure_token();

    dbg!(token);
    
    Ok(Json(AuthBody::new(token.to_owned())))
        
}


pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<PersonData>,
) -> Result<Json<Option<RecordUser>>, DbError> {

    tracing::info!("Creating user: {:?}", payload);

    let db = &state.db;
    let name = payload.name;
    let pass = payload.pass;
    let email = payload.email;
    let username = payload.username;

    let jwt = db
        .signup(Record {
            access: "account",
            namespace: &*NAMESPACE,
            database: &*DATABASE,
            params: Params {
                name: &name,
                pass: &pass,
                username: &username,
                email: &email,
            },
        })
        .await?;

        let token = jwt.as_insecure_token();
        dbg!(token);

    tracing::info!("User created: {:?}", jwt);

 Ok(Json(Some(RecordUser {
        name,
        email,
        username,
        pass,
        jwt: token.to_string(),
 })))
    
}
