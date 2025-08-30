use axum::{http::StatusCode, response::Html};

pub async fn admin_login_page() -> Result<Html<String>, StatusCode> {
    let html = include_str!("../../assets/login.html");
    Ok(Html(html.to_string()))
}
