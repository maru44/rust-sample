use chrono::{Date, DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserInput {
    pub username: String,
    pub email: String,
}
