mod app_data;
mod config;
mod error;
mod game;
mod health;
mod movie;
mod music;

use actix_web::{App, HttpServer};
use error::Result;

use health::health_check;

#[actix_web::main]
async fn main() -> Result<()> {
    tokio::spawn(async { println!("hai") });
    HttpServer::new(|| App::new().service(health_check))
        .bind(("127.0.0.1", 8080))
        .expect("actix server could not bind to address")
        .run()
        .await
        .expect("actix server could not run");
    Ok(())
}
