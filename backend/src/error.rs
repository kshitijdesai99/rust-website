use thiserror::Error;
use sea_orm::{DbErr, RuntimeErr};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] DbErr),
    
    #[error("Runtime error: {0}")]
    Runtime(#[from] RuntimeErr),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Not found")]
    NotFound,
}

pub type Result<T> = std::result::Result<T, Error>;