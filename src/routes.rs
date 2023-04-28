mod health;
mod middlewares;
mod register;
mod login;
mod profile;
use health::health_checker_handler;
use register::register_user_handler;
use login::login_handler;
use profile::get_profile_handler;
use middlewares::auth_guard;

use crate::{
    config::Config,
    app_state::AppState
};

use std::sync::Arc;
use sqlx::postgres::PgPoolOptions;
use axum::{
    Router,
    middleware,
    routing::{get, post},
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    }
};
use tower_http::cors::CorsLayer;

pub async fn make_app() -> Router {
    let config = Config::init();
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    let state = Arc::new(AppState {
        db: pool.clone(),
        env: config.clone(),
    });
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    let app = Router::new()
        .route("/api/health", get(health_checker_handler))
        .route("/api/register", post(register_user_handler))
        .route("/api/login", post(login_handler))
        .route(
            "/api/profile",
            get(get_profile_handler)
                .route_layer(
                    middleware::from_fn_with_state(state.clone(), auth_guard)),
        )
        .with_state(state)
        .layer(cors);
    return app;
}
