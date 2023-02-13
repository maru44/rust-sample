use once_cell::sync::Lazy;
use sqlx::{postgres::PgPoolOptions, Error, PgConnection, PgPool};

struct Config {
    postgres_host: String,
    postgres_port: String,
    postgres_user: String,
    postgres_password: String,
    postgres_database: String,
}

impl Config {
    pub fn database_url(&self) -> String {
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
        return Pg { pool };
    }
}

pub async fn new_pg() -> Result<Pg, Error> {
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(CONFIG.database_url().as_str())
        .await;

    match pool {
        Ok(p) => Ok(Pg::new(p)),
        Err(e) => Err(e),
    }
}

// pub fn newPgConnection(c: &Config) -> PgConnection {

// }
