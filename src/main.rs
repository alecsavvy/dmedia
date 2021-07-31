mod app_data;
mod config;
mod error;
mod game;
mod movie;
mod music;

use actix_web::{App, HttpServer};
use error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", 8080))
        .expect("actix server could not bind to address")
        .run()
        .await
        .expect("actix server could not run");
    Ok(())
}
