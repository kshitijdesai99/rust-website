/// Backend application library
pub mod config;
pub mod database;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod repositories;
pub mod services;
pub mod utils;

pub use config::Config;
pub use errors::{AppError, Result};
