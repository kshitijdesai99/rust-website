use axum::{
    extract::{Path, Query, State, ConnectInfo},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder};
use crate::{entities::{blogs, blog_views}, AppState};
use std::net::SocketAddr;

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct BlogListItem {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub excerpt: Option<String>,
    pub status: Option<String>,
    pub published_at: Option<sea_orm::prelude::DateTimeUtc>,
}

#[derive(Debug, Serialize)]
pub struct BlogsListResponse {
    pub items: Vec<BlogListItem>,
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
    pub total_pages: u64,
}

pub async fn list_blogs(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(10).clamp(1, 100);
    let status = params.status.unwrap_or_else(|| "published".to_string());

    let mut query = blogs::Entity::find()
        .filter(blogs::Column::Slug.is_not_null())
        .order_by_desc(blogs::Column::PublishedAt)
        .order_by_desc(blogs::Column::CreatedAt);

    // Filter by status if provided (defaults to published)
    if !status.is_empty() {
        query = query.filter(blogs::Column::Status.eq(status));
    }

    let paginator = query.paginate(&state.db, per_page);

    let total = match paginator.num_items().await {
        Ok(t) => t as u64,
        Err(_) => 0,
    };
    let total_pages = paginator.num_pages().await.unwrap_or(0) as u64;

    let page_idx = page - 1; // SeaORM paginator is 0-based
    let models = paginator.fetch_page(page_idx).await.unwrap_or_default();

    let items = models
        .into_iter()
        .map(|m| BlogListItem {
            id: m.id,
            slug: m.slug,
            title: m.title,
            excerpt: m.excerpt,
            status: m.status,
            published_at: m.published_at,
        })
        .collect::<Vec<_>>();

    Json(BlogsListResponse {
        items,
        page,
        per_page,
        total,
        total_pages,
    })
}

#[derive(Debug, Serialize)]
pub struct BlogDetailResponse {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content: String,
    pub status: Option<String>,
    pub published_at: Option<sea_orm::prelude::DateTimeUtc>,
    pub created_at: Option<sea_orm::prelude::DateTimeUtc>,
    pub updated_at: Option<sea_orm::prelude::DateTimeUtc>,
    pub views_count: u64,
}

pub async fn get_blog_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    match blogs::Entity::find()
        .filter(blogs::Column::Slug.eq(slug))
        .one(&state.db)
        .await
    {
        Ok(Some(m)) => {
            // Record a view (best-effort)
            let ip = addr.ip().to_string();
            let view = blog_views::ActiveModel {
                post_id: sea_orm::ActiveValue::set(m.id),
                ip_address: sea_orm::ActiveValue::set(ip),
                timestamp: sea_orm::ActiveValue::set(chrono::Utc::now()),
                ..Default::default()
            };
            let _ = blog_views::Entity::insert(view).exec(&state.db).await;

            // Count total views
            let views_count = blog_views::Entity::find()
                .filter(blog_views::Column::PostId.eq(m.id))
                .count(&state.db)
                .await
                .unwrap_or(0);

            let resp = BlogDetailResponse {
                id: m.id,
                author_id: m.author_id,
                title: m.title,
                slug: m.slug,
                excerpt: m.excerpt,
                content: m.content,
                status: m.status,
                published_at: m.published_at,
                created_at: m.created_at,
                updated_at: m.updated_at,
                views_count,
            };
            Json(resp).into_response()
        }
        Ok(None) => (
            axum::http::StatusCode::NOT_FOUND,
            Json(json!({ "error": "Blog not found" })),
        )
            .into_response(),
        Err(_) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal server error" })),
        )
            .into_response(),
    }
}
