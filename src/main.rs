mod app_state;
mod config;
mod models;
mod routes;
pub use app_state::AppState;
pub use config::Config;

use crate::routes::make_app;
use dotenv::dotenv;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let app = make_app().await?;
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("🚀 Server started successfully");
    axum::serve(listener, app).await?;
    Ok(())
}
