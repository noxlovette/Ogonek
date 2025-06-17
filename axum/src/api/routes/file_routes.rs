use crate::api::files::file;
use crate::schema::AppState;
use axum::routing::get;
use axum::Router;

pub fn file_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(file::list_files))
        .route("/presigned/{encoded_key}", get(file::fetch_presigned_url))
        .route(
            "/{file_id}",
            get(file::fetch_file)
                .patch(file::update_file)
                .delete(file::delete_file),
        )
}
