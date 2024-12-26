mod auth;
mod endpoints;

use log4rs;

use actix_web::{App, HttpServer, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok(); 
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let _ = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .configure(endpoints::config)
    })
    .bind("localhost:8080")?
    .run()
    .await
}