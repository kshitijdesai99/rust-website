pub mod admin;
pub mod api;
pub mod auth;
pub mod entities;
pub mod error;
pub mod graphql;
pub mod migration;

use axum::Router;
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::net::SocketAddr;
use tracing_subscriber::fmt::init;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    init();
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://./data.db?mode=rwc".to_string());
    
    let db = Database::connect(&database_url).await?;
    migration::Migrator::up(&db, None).await?;
    
    let state = AppState { db };
    let app = create_app(state);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Server running on http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;
    
    Ok(())
}

fn create_app(state: AppState) -> Router {
    api::routes::create_routes(state)
}