// this module will interact with the database directly. if there is a connection involved, it should be here
pub mod auth;

// initiate the DB and set it as static
use std::sync::Arc;
use surrealdb::{
    engine::remote::ws::{Client, Wss},
    opt::auth::Root,
    Surreal,
};
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Surreal<Client>>,
}

// pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

pub async fn init_db() -> Result<Arc<Surreal<Client>>, Box<dyn std::error::Error>> {
    let db: Arc<Surreal<Client>> = Arc::new(Surreal::new::<Wss>("db.noxlovette.com").await?);

    db.signin(Root {
        username: "firelight",
        password: "firelight",
    })
    .await?;

    db.use_ns("namespace").use_db("database").await?;

    Ok(db)
}

// convert DB errors into a response
mod error {
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use axum::response::Response;
    use axum::Json;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("database error")]
        Db,
    }

    impl IntoResponse for Error {
        fn into_response(self) -> Response {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
        }
    }

    impl From<surrealdb::Error> for Error {
        fn from(error: surrealdb::Error) -> Self {
            eprintln!("{error}");
            Self::Db
        }
    }
}
