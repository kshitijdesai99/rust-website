use axum::{
    routing::get,
    Router,
};
use backend::{
    config::Config,
    database::create_pool,
    handlers::{health_check, create_user, delete_user, get_user, get_users, update_user},
    repositories::UserRepository,
    services::UserService,
    utils::{create_cors, setup_tracing},
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    setup_tracing();
    
    tracing::info!("Starting backend application");

    // Load configuration
    let config = Config::from_env();
    tracing::info!("Configuration loaded: {:?}", config);

    // Setup database connection
    let pool = create_pool(&config.database_url).await?;
    tracing::info!("Database connection established");

    // Initialize repositories and services
    let user_repository = UserRepository::new(pool);
    let user_service = UserService::new(user_repository);

    // Setup routes
    let app = Router::new()
        // Health check
        .route("/api/health", get(health_check))
        // User routes
        .route("/api/users", get(get_users).post(create_user))
        .route(
            "/api/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        // Middleware
        .layer(create_cors())
        .with_state(user_service);

    // Start server
    let addr: SocketAddr = config.server_address().parse()?;
    tracing::info!("Backend server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
