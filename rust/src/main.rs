mod auth;
mod endpoints;
use actix_web::{middleware::Logger, App, HttpServer};
use log4rs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
   

    let _ = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(endpoints::config)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
