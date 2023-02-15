use chrono::NaiveDateTime;
use serde::Deserialize;
use sqlx::FromRow;

use crate::utils::error::MyError;

use super::pg::new_pg;

#[derive(Debug, FromRow)]
pub struct UserRow {
    pub id: sqlx::types::Uuid,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserInput {
    pub username: String,
    pub email: String,
}

pub async fn create_user(input: UserInput) -> Result<(), MyError> {
    let pool = new_pg().await?;

    sqlx::query(
        r#"
INSERT INTO users (username, email) VALUES ($1, $2);
    "#,
    )
    .bind(input.username)
    .bind(input.email)
    .execute(&pool.pool)
    .await?;

    Ok(())
}

pub async fn list_users() -> Result<Vec<UserRow>, MyError> {
    let pg = new_pg().await?;

    let us = sqlx::query_as::<_, UserRow>("SELECT * FROM users;")
        .fetch_all(&pg.pool)
        .await?;

    Ok(us)
}

pub async fn get_user(user_id: uuid::Uuid) -> Result<Option<UserRow>, MyError> {
    let pg = new_pg().await?;

    let u: Option<UserRow> = sqlx::query_as("SELECT * FROM users WHERE users.id = $1;")
        .bind(user_id)
        .fetch_optional(&pg.pool)
        .await?;

    Ok(u)
}
