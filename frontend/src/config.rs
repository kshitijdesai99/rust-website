/// Frontend configuration
/// API base URL configuration for the frontend
/// This reads from the API_BASE_URL environment variable at build time
pub const API_BASE_URL: &str = match option_env!("API_BASE_URL") {
    Some(url) => url,
    None => "http://127.0.0.1:3000/api",
};
