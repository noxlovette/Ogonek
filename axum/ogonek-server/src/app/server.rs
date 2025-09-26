use crate::{api::routes::root, schema::AppState, services::init_tracing};

pub async fn server() -> anyhow::Result<()> {
    init_tracing().await?;
    let state = AppState::new().await?;
    let cors = std::env::var("CORS").expect("CORS needs to be set");
    let app = root(state, cors)?;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("🚀 Server starting on http://0.0.0.0:3000");

    axum::serve(listener, app).await?;
    Ok(())
}
