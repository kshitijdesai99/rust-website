// This file contains helper functions and utilities that multiple tests can use
// Think of it as a toolbox of useful functions for testing

use sqlx::SqlitePool;

// This function creates a test database in memory (not saved to disk)
// It's perfect for testing because:
// 1. It's fast (everything happens in RAM)
// 2. It's clean (starts fresh for each test)
// 3. It doesn't interfere with your real database
pub async fn setup_test_db() -> SqlitePool {
    // Create a database connection that only exists in memory
    let pool = SqlitePool::connect(":memory:")
        .await
        .expect("Failed to create test database");
    
    // Run all our migrations on the test database
    // This gives us the same table structure as our real database
    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    
    pool  // Return the database connection for tests to use
}