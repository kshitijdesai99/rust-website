use gloo_net::http::Request;
use serde::Deserialize;
use crate::config::API_BASE_URL;
use chrono;

#[derive(Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
}

#[derive(Deserialize)]
pub struct ApiStatusResponse {
    pub service: String,
    pub version: String,
    pub status: String,
}

pub struct ApiService;

impl ApiService {
    pub async fn health_check() -> Result<HealthResponse, String> {
        let response = Request::get(&format!("{}/health", API_BASE_URL))
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let health_data: HealthResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(health_data)
    }

    pub async fn get_api_status() -> Result<ApiStatusResponse, String> {
        let response = Request::get(&format!("{}/api/v1/status", API_BASE_URL))
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let status_data: ApiStatusResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(status_data)
    }

    // Blogs API
    pub async fn list_blogs(page: u64, per_page: u64) -> Result<BlogsListResponse, String> {
        let url = format!("{}/api/blogs?page={}&per_page={}", API_BASE_URL, page, per_page);
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let data: BlogsListResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(data)
    }

    pub async fn get_blog_by_slug(slug: &str) -> Result<BlogDetailResponse, String> {
        let url = format!("{}/api/blogs/{}", API_BASE_URL, slug);
        let response = Request::get(&url)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.ok() {
            return Err(format!("Request failed with status: {}", response.status()));
        }

        let data: BlogDetailResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(data)
    }
}

// Blog models
#[derive(Deserialize, Clone, PartialEq)]
pub struct BlogListItem {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub excerpt: Option<String>,
    pub status: Option<String>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,    
}

#[derive(Deserialize, Clone)]
pub struct BlogsListResponse {
    pub items: Vec<BlogListItem>,
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
    pub total_pages: u64,
}

#[derive(Deserialize, Clone)]
pub struct BlogDetailResponse {
    pub id: i32,
    pub author_id: i32,
    pub title: String,
    pub slug: String,
    pub excerpt: Option<String>,
    pub content: String,
    pub status: Option<String>,
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,    
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,    
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,    
}
