use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("{0:?}")]
    Str(String),
    #[error(transparent)]
    Wrapped(anyhow::Error),
}
