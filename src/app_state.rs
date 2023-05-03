use crate::config::Config;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
type Db = Surreal<Client>;

pub struct AppState {
    pub db: Db,
    pub config: Config,
}
