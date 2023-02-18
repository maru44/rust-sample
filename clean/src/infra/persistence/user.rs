use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::FromRow;

use crate::domain::{
    error::DError,
    model::user::{User, UserId, UserInput},
    repository,
};

use super::pg::Pg;

#[derive(Debug, FromRow, Clone)]
struct UserRow {
    pub id: sqlx::types::Uuid,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl UserRow {
    fn to(self) -> User {
        return User {
            id: UserId {
                id: uuid::Uuid::from_u128(self.id.as_u128()),
            },
            username: self.username,
            email: self.email,
            created_at: self.created_at,
            updated_at: self.updated_at,
        };
    }
}

pub struct UserRepository {
    pg: Pg,
}

impl UserRepository {
    pub fn new(pg: Pg) -> Self {
        Self { pg }
    }
}

#[async_trait]
impl repository::user::UserRepository for UserRepository {
    async fn create(&self, input: UserInput) -> Result<(), DError> {
        sqlx::query(
            "
            INSERT INTO users (username, email) VALUES ($1, $2);
        ",
        )
        .bind(input.username)
        .bind(input.email)
        .execute(&self.pg.pool)
        .await?;

        Ok(())
    }

    async fn list(&self) -> Result<Vec<User>, DError> {
        let us = sqlx::query_as::<_, UserRow>("SELECT * FROM users;")
            .fetch_all(&self.pg.pool)
            .await?;

        let users = us.into_iter().map(|u| u.to()).collect::<Vec<User>>();
        Ok(users)
    }

    async fn get(&self, id: &UserId) -> Result<User, DError> {
        let res = sqlx::query("SELECT * FROM users WHERE users.id = $1;")
            .bind(id.id)
            .fetch_one(&self.pg.pool)
            .await?;

        let user_row = UserRow::from_row(&res)?;
        Ok(user_row.to())
    }
}
