use crate::config::Config;
use sqlx::Pool;
use sqlx::Postgres;

pub struct AppState {
    pub db: Pool<Postgres>,
    pub env: Config,
}
