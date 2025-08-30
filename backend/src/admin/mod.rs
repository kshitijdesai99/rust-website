pub mod login;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    Json,
};
use async_graphql::{http::GraphQLPlaygroundConfig, http::playground_source};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use sea_orm_pro::{ConfigParser, JsonCfg};
use tracing::error;
use crate::{auth, graphql, AppState};

pub use login::admin_login_page;

pub async fn admin_panel_config() -> Result<Json<JsonCfg>, (StatusCode, &'static str)> {
    let config = ConfigParser::new()
        .load_config("pro_admin")
        .map_err(|e| {
            error!("Failed to load admin config: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to load admin config")
        })?;
    
    Ok(Json(config))
}

pub async fn graphql_playground() -> impl IntoResponse {
    let config = GraphQLPlaygroundConfig::new("/api/graphql")
        .with_header("Authorization", "Bearer TOKEN_HERE");
    
    let html = playground_source(config).replace(
        r#""Authorization":"Bearer TOKEN_HERE""#,
        r#""Authorization":`Bearer ${localStorage.getItem('auth_token')}`"#,
    );
    
    Html(html)
}

pub async fn graphql_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> Result<GraphQLResponse, (StatusCode, &'static str)> {
    let request = req.into_inner();
    
    // Allow introspection queries without authentication
    let is_introspection = request.query.contains("__schema") || request.query.contains("__type");
    
    let (depth, complexity) = if is_introspection {
        (None, None)
    } else {
        (Some(10), Some(1000))
    };

    let schema = graphql::schema(state.db.clone(), depth, complexity)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create GraphQL schema"))?;

    if !is_introspection {
        auth::check_user_auth(&headers)?;
    }

    let result = schema.execute(request).await;
    Ok(result.into())
}
