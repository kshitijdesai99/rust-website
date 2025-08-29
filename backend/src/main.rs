use backend::{
    config::Config,
    database::create_pool,
    routes::app_router,
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
    let app = app_router()
        .layer(create_cors())
        .with_state(user_service);

    // Start server
    let addr: SocketAddr = config.server_address().parse()?;
    tracing::info!("Backend server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
