mod auth;
mod endpoints;
use rust::db::postgres::pool::init_pool; 
use std::sync::Arc;
use log4rs;
use actix_web::{App, HttpServer, middleware::Logger, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok(); 
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let pool = Arc::new(init_pool(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")));

    // Try to get a connection from the pool to verify connection
    match get_conn(&pool) {
        Ok(conn) => {
            println!("Successfully connected to the database.");
            // You might want to perform a simple query here to verify further
            // Example:
            // diesel::sql_query("SELECT 1").execute(&*conn).expect("Failed to execute query");
        },
        Err(e) => {
            eprintln!("Failed to connect to the database: {:?}", e);
            // Depending on your application requirements, you might want to exit here
            // or handle the error in a more sophisticated manner
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Database connection error"));
        }
    }


    let _ = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Share the pool across all instances
            .wrap(Logger::default())
            .configure(endpoints::config)
    })
    .bind("localhost:8080")?
    .run()
    .await
}