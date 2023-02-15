use anyhow::{Error, Ok, Result};
use sqlx::{
    postgres::{PgPool, PgRow},
    Row,
};

use crate::model::user::{User, UserInput};

// use sqlx::postgres::

// async fn users(
//     Extension(db): Extension<
// )

pub async fn create_user(input: UserInput, pool: &PgPool) -> Result<()> {
    sqlx::query(
        "
        INSERT INTO users (name, email) VALUES ($1, $2);
    ",
    )
    .bind(input.name)
    .bind(input.email)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list_users(pool: &PgPool) -> Result<Vec<User>> {
    Ok(sqlx::query(
        r#"
SELECT * FROM users;
        "#,
    )
    .map(|row: PgRow| User {
        id: uuid::Uuid::from_u128(row.get::<sqlx::types::Uuid, _>(0).as_u128()),
        name: row.get(1),
        email: row.get(2),
        created_at: row.get(3),
        updated_at: row.get(4),
    })
    .fetch_all(pool)
    .await?)
}
