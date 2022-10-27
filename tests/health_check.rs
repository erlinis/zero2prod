//! tests/health_check.rs

use std::{net::TcpListener, fmt::format};

#[tokio::test]
async fn health_check_works() {
    //Start the application
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    //assert!
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // We retrieve the port assigned to the app by the OS
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind adress");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
