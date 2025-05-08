use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrmError {
    #[error("Postgres error: {0}")]
    Postgres(#[from] postgres::Error),

    #[error("Unknown error")]
    Unknown,
}

pub type OrmResult<T> = Result<T, OrmError>;