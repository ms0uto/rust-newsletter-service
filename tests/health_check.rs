use reqwest::Client;

#[tokio::test]
async fn health_works() {
    spawn_app();

    let client = Client::new();
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}

fn spawn_app() {
    let server = newsletter_service::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}