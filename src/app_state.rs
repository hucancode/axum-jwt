use crate::config::Config;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub struct AppState {
    pub db: Surreal<Client>,
    pub config: Config,
}
