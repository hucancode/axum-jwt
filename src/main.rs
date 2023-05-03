mod app_state;
mod config;
mod models;
mod routes;
pub use app_state::AppState;
pub use config::Config;

use crate::routes::make_app;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let app = make_app().await?;
    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
