mod bus_route;
mod bus_stop;
mod vehicle;
mod card;
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
use std::error::Error;
use std::sync::Arc;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tower_http::cors::CorsLayer;

pub async fn make_app() -> Result<Router, Box<dyn Error>> {
    let config = Config::init();
    println!("connecting to surrealdb... at {}", config.db_url);
    let db = Surreal::new::<Ws>(config.db_url.clone()).await?;
    println!("logging in to surrealdb!");
    db.signin(Root {
        username: &config.db_user,
        password: &config.db_password,
    })
    .await?;
    db.use_ns(&config.db_namespace)
        .use_db(&config.db_name)
        .await?;
    println!("connected to surrealdb!");
    let cors = HeaderValue::from_str(&config.cors_url)?;
    let state = Arc::new(AppState { db, config });
    let cors = CorsLayer::new()
        .allow_origin(cors)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    let ret = Router::new()
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
        .route("/api/bus-stops", get(bus_stop::get_all_handler))
        .route("/api/bus-stops/{id}", get(bus_stop::get_handler))
        .route(
            "/api/bus-stops",
            post(bus_stop::create_handler)
            //.route_layer(middleware::from_fn_with_state(state.clone(), auth_guard)),
        )
        .route("/api/bus-routes", get(bus_route::get_all_handler))
        .route("/api/bus-routes/{id}", get(bus_route::get_handler))
        .route(
            "/api/bus-routes/{id}/add-stop/{stop_id}",
            post(bus_route::add_stop_handler),
        )
        .route(
            "/api/bus-routes/{id}/add-vehicle/{stop_id}",
            post(bus_route::add_vehicle_handler),
        )
        .route(
            "/api/bus-routes",
            post(bus_route::create_handler)
            //.route_layer(middleware::from_fn_with_state(state.clone(), auth_guard)),
        )
        .route("/api/vehicles", get(vehicle::get_all_handler))
        .route("/api/vehicles/{id}", get(vehicle::get_handler))
        .route(
            "/api/vehicles",
            post(vehicle::create_handler)
            //.route_layer(middleware::from_fn_with_state(state.clone(), auth_guard)),
        )
        .route("/api/vehicles/{id}/location",
            post(vehicle::update_location_handler)
            //.route_layer(middleware::from_fn_with_state(state.clone(), auth_guard)),
        )
        .route("/api/cards", get(card::get_all_cards_handler))
        .route("/api/cards/{id}", get(card::get_card_handler))
        .route(
            "/api/cards",
            post(card::create_card_handler)
            //.route_layer(middleware::from_fn_with_state(state.clone(), auth_guard)),
        )
        .route(
            "/api/cards/{id}/recharge",
            post(card::recharge_card_handler)
            //.route_layer(middleware::from_fn_with_state(state.clone(), auth_guard)),
        )
        .route("/api/passes", get(card::get_all_passes_handler))
        .route("/api/passes/{id}", get(card::get_pass_handler))
        .route(
            "/api/passes",
            post(card::create_pass_handler)
            //.route_layer(middleware::from_fn_with_state(state.clone(), auth_guard)),
        )
        .route(
            "/api/cards/{id}/validate/{stop_id}",
            post(card::validate_card_at_stop_handler)
            //.route_layer(middleware::from_fn_with_state(state.clone(), auth_guard)),
        )
        .with_state(state)
        .layer(cors);

    Ok(ret)
}
