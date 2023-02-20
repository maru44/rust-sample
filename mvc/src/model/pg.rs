use crate::utils::error::MyError;
use once_cell::sync::Lazy;
use sqlx::{postgres::PgPoolOptions, PgPool};

struct Config {
    postgres_host: String,
    postgres_port: String,
    postgres_user: String,
    postgres_password: String,
    postgres_database: String,
}

impl Config {
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_database,
        )
    }
}

static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    postgres_user: std::env::var("POSTGRES_USER").unwrap(),
    postgres_password: std::env::var("POSTGRES_PASSWORD").unwrap(),
    postgres_host: std::env::var("POSTGRES_HOST").unwrap(),
    postgres_port: std::env::var("POSTGRES_PORT").unwrap(),
    postgres_database: std::env::var("POSTGRES_DATABASE").unwrap(),
});

#[derive(Debug, Clone)]
pub struct Pg {
    pub pool: PgPool,
}

impl Pg {
    fn new(pool: PgPool) -> Self {
        Pg { pool }
    }
}

pub async fn new_pg() -> anyhow::Result<Pg, MyError> {
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(CONFIG.url().as_str())
        .await;

    match pool {
        Ok(p) => Ok(Pg::new(p)),
        Err(e) => Err(MyError::from(e)),
    }
}

impl From<sqlx::Error> for MyError {
    fn from(error: sqlx::Error) -> Self {
        MyError::Wrapped(anyhow::Error::new(error))
    }
}

// #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
// struct Uuid(uuid::Uuid);

// impl From<sqlx::types::Uuid> for Uuid {
//     fn from(value: sqlx::types::Uuid) -> Self {
//         Uuid(uuid::Uuid::from_u128(value.as_u128()))
//     }
// }
