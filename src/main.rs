mod routes;
mod app_state;
mod config;
mod models;
pub use app_state::AppState;
pub use config::Config;

use dotenv::dotenv;
use crate::routes::make_app;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = make_app()
        .await;
    println!("🚀 Server started successfully");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
