pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Secret not found: {0}")]
    SecretNotFound(String),
    #[error("Postgres Connection Error: {0}")]
    PostgresConnectionError(String),
}