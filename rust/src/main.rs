use axum::routing::{get, post};
use axum::Router;
use rust::db::init::init_db;
use rust::db::init::AppState;
use rust::tools::logging::init_logging;

// use rust::api::user::{delete_user, fetch_user, list_users, update_user};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_logging().await;
    let state = AppState {
        db: init_db().await?,
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/signup", post(rust::api::auth::signup))
        .route("/signin", post(rust::api::auth::authorize))
        .route("/user", get(rust::api::user::fetch_user_self))
        .route(
            "/user/:id",
            get(rust::api::user::fetch_user)
                .post(rust::api::user::update_user)
                .delete(rust::api::user::delete_user),
        )
        .route("/users", get(rust::api::user::list_users))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
