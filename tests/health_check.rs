mod utils;

use utils::spawn_app;

#[tokio::test]
async fn health_check_returns_200() {
    // Arrange
    let address = spawn_app().await;

    // Act
    let res = reqwest::get(&format!("{}/health_check", &address)).await.expect("failed to send get request");

    // Assert
    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}
