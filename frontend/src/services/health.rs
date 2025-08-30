use gloo_net::http::Request;
use serde::Deserialize;
use crate::config::API_BASE_URL;

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
