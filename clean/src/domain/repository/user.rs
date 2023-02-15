use async_trait::async_trait;

use crate::domain::{
    error::DError,
    model::user::{User, UserId},
};

#[async_trait]
pub trait UserRepository: Send + Sync + 'static {
    async fn create(&self, user: &User) -> Result<(), DError>;
    async fn get(&self, id: &UserId) -> Result<User, DError>;
    async fn list(&self) -> Result<Vec<User>, DError>;
}
