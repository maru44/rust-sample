use thiserror::Error;

#[derive(Debug, Error)]
pub enum DError {
    #[error("{0:?}")]
    Str(String),
    #[error(transparent)]
    Wrapped(anyhow::Error),
}
