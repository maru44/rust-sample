use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UserId {
    id: uuid::Uuid,
}

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserInput {
    pub username: String,
    pub email: String,
}
