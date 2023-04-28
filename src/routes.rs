mod health;
mod login;
mod middlewares;
mod profile;
mod register;
use health::health_checker_handler;
use login::login_handler;
use middlewares::auth_guard;
use profile::get_profile_handler;
use register::register_user_handler;

use crate::{AppState, Config};

use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    middleware,
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub async fn make_app() -> Router {
    let config = Config::init();
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .unwrap_or_else(|e| {
            println!("🔥 Failed to connect to the database: {:?}", e);
            std::process::exit(1);
        });
    let state = Arc::new(AppState {
        db: pool,
        env: config,
    });
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    Router::new()
        .route("/", get(health_checker_handler))
        .route("/api", get(health_checker_handler))
        .route("/api/health", get(health_checker_handler))
        .route("/api/register", post(register_user_handler))
        .route("/api/login", post(login_handler))
        .route(
            "/api/profile",
            get(get_profile_handler)
                .route_layer(middleware::from_fn_with_state(state.clone(), auth_guard)),
        )
        .with_state(state)
        .layer(cors)
}
