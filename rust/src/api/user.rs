use crate::db::AppState;
use crate::db::error::Error as DbError;
use crate::db::{DATABASE, NAMESPACE};
use surrealdb::opt::auth::Record;
use surrealdb::RecordId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonData {
    pub name: String,
    pub email: String,
    pub pass: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordUser {
    name: String,
    pass: String,
    email: String,
    jwt: String,
}

#[derive(Serialize, Deserialize)]
struct Params<'a> {
    name: &'a str,
    pass: &'a str,
    email: &'a str,
}

use axum::extract::Json;
use axum::extract::State;

const PERSON: &str = "person";

pub async fn create_user_endpoint(
    State(state): State<AppState>,
    Json(payload): Json<PersonData>,
) -> Result<Json<Option<RecordUser>>, DbError> {

    tracing::info!("Creating user: {:?}", payload);

    let db = &state.db;
    let name = payload.name;
    let pass = payload.pass;
    let email = payload.email;

    let jwt = db
        .signup(Record {
            access: "account",
            namespace: &*NAMESPACE,
            database: &*DATABASE,
            params: Params {
                name: &name,
                pass: &pass,
                email: &email,
            },
        })
        .await?
        .into_insecure_token();

    tracing::info!("User created: {:?}", jwt);

 Ok(Json(Some(RecordUser {
        name,
        email,
        pass,
        jwt,
 })))
    
}
