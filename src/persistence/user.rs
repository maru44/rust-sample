use anyhow::{Error, Ok, Result};
use sqlx::postgres::PgPool;

use crate::model::user::{User, UserInput};

// use sqlx::postgres::

// async fn users(
//     Extension(db): Extension<
// )

pub async fn create_user(input: UserInput, pool: &PgPool) -> Result<()> {
    sqlx::query(
        "
        INSERT INTO users (username, email) VALUES ($1, $2);
    ",
    )
    .bind(input.username)
    .bind(input.email)
    .fetch_one(pool)
    .await?;

    Ok(())
}

pub async fn list_users(pool: &PgPool) -> Result<Vec<User>> {
    Ok(sqlx::query_as(
        r#"
SELECT * FROM users;
        "#,
    )
    .fetch_all(pool)
    .await?)
}
