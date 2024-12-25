mod auth;
mod endpoints;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok(); 
    let _ = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    HttpServer::new(|| {
        App::new()
            .configure(endpoints::config)
    })
    .bind("localhost:8080")?
    .run()
    .await
}