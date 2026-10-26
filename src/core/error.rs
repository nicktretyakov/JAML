use thiserror::Error;

#[derive(Error, Debug)]
pub enum CmsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Template error: {0}")]
    Template(String),

    #[error("Content not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Build error: {0}")]
    Build(String),

    #[error("Concurrency error: {0}")]
    Concurrency(String),

    #[error("Anyhow: {0}")]
    Anyhow(#[from] anyhow::Error),
}

pub type CmsResult<T> = Result<T, CmsError>;
