#[derive(Debug, Clone)]
pub struct Config {
    pub db_user: String,
    pub db_password: String,
    pub db_url: String,
    pub db_namespace: String,
    pub db_name: String,
    pub jwt_secret: String,
    pub jwt_expires_in: String,
    pub jwt_maxage: i32,
}

impl Config {
    pub fn init() -> Config {
        let db_user = std::env::var("DATABASE_USER").expect("DATABASE_USER must be set");
        let db_password =
            std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_namespace = std::env::var("DATABASE_NAMESPACE").unwrap_or(String::from("app"));
        let db_name = std::env::var("DATABASE_NAME").unwrap_or(String::from("master"));
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");
        Config {
            db_user,
            db_password,
            db_url,
            db_namespace,
            db_name,
            jwt_secret,
            jwt_expires_in,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
        }
    }
}
