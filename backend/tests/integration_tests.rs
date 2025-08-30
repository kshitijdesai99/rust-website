// This file contains integration tests for our API
// Integration tests check that different parts of our app work together correctly
// Unlike unit tests (which test one function), these test the whole system

mod common;  // Import our shared test utilities

use axum_test::TestServer;  // A tool to test web applications

// This test checks that our health endpoint works correctly
#[tokio::test]  // This tells Rust this is an async test
async fn test_health_check() {
    // Create a test version of our web application
    let app = backend::create_app().await;
    
    // Start a test server with our app
    let server = TestServer::new(app).unwrap();
    
    // Make a GET request to the /health endpoint
    let response = server.get("/health").await;
    
    // Check that we get a successful response (status code 200)
    assert_eq!(response.status_code(), 200);
}