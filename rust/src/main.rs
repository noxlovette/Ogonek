mod auth;
mod endpoints;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(endpoints::config)
    })
    .bind("localhost:8080")?
    .run()
    .await
}