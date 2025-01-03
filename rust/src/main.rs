use axum::routing::{get, post};
use axum::Router;
use rust::db::init_db;
use rust::db::AppState;
use rust::tools::logging::init_logging;

use rust::api::user::{delete_user, fetch_user, list_users, update_user};

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
        .route(
            "/user/:id",
            get(fetch_user).put(update_user).delete(delete_user),
        )
        .route("/users", get(list_users))
        .route(
            "/lesson/:id",
            get(rust::api::lesson::fetch_lesson)
                .put(rust::api::lesson::update_lesson)
                .delete(rust::api::lesson::delete_lesson),
        )
        .route("/lessons", get(rust::api::lesson::list_lessons))
        .route("/bookmarks", get(rust::api::bookmark::list_bookmarks))
        .route(
            "/bookmark/:id",
            get(rust::api::bookmark::get_bookmark)
                .patch(rust::api::bookmark::update_bookmark)
                .delete(rust::api::bookmark::delete_bookmark),
        )
        .route("/bookmark", post(rust::api::bookmark::create_bookmark))
        .route("/lesson", post(rust::api::lesson::create_lesson))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("localhost:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}
