//! Integration tests for the backend application.
//!
//! This module contains integration tests that verify the correctness of the
//! application's API endpoints. It includes a health check test to ensure the
//! server is up and running.

/// Integration test for the health check endpoint.
///
/// This test verifies that the `/health_check` endpoint is functioning correctly
/// by spawning a test instance of the application server and making an HTTP GET
/// request to the health check endpoint.
///
/// # Test Behavior
///
/// The test performs the following checks:
/// 1. Spawns the application server on `http://127.0.0.1:8000`
/// 2. Makes a GET request to `/health_check`
/// 3. Verifies that the response status is successful (2xx)
/// 4. Verifies that the response body has zero content length (empty body)
///
/// # Assertions
///
/// * Response status is successful (200 OK)
/// * Response content length is 0 (no body)
///
/// # Panics
///
/// This test will panic if:
/// - The application fails to spawn
/// - The HTTP request fails to execute
/// - Any assertion fails
#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

/// Spawns the application server for testing purposes.
///
/// This helper function starts an instance of the backend server in the background
/// using Tokio's spawn mechanism. The server runs on `http://127.0.0.1:8000` and
/// continues running in the background for the duration of the test.
///
/// # Panics
///
/// This function will panic if the server fails to initialize or start, typically
/// due to:
/// - Port 8000 already being in use
/// - Database connection failure
/// - Missing required environment variables
///
/// # Note
///
/// The spawned server is not explicitly shut down after the test completes, as it
/// runs in a background task that will be terminated when the test process exits.
async fn spawn_app() {
    let server = backend::run().await.expect("Failed to spawn our app.");
    tokio::spawn(server);
}