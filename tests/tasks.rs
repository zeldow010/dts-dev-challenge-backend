mod utils;

use chrono::{DateTime, Utc};
use fake::{Dummy, Fake, Faker};
use serde::Serialize;
use utils::spawn_app;
use uuid::Uuid;

#[derive(Debug, Dummy, Serialize)]
struct TaskPayload {
    title: String,
    description: Option<String>,
    due: DateTime<Utc>,
}

#[tokio::test]
async fn add_task_to_db_and_return_id() {
    // Arrange
    let address = spawn_app().await;

    let client = reqwest::Client::new();

    let new_task: TaskPayload = Faker.fake();

    // Act
    let res = client
        .post(format!("{}/tasks", address))
        .header("Content-Type", "application/json")
        .json(&new_task)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, res.status().as_u16());
}

#[tokio::test]
async fn success_delete_task() {
    // Arrange
    let address = spawn_app().await;

    let client = reqwest::Client::new();

    let new_task: TaskPayload = Faker.fake();

    let res = client
        .post(format!("{}/tasks", address))
        .header("Content-Type", "application/json")
        .json(&new_task)
        .send()
        .await
        .expect("Failed to execute request.");

    let delete_id: Uuid = res.json().await.unwrap();

    // Act

    let res = client
        .delete(format!("{}/tasks/{}", address, &delete_id))
        .header("Content-Type", "application/json")
        .json(&delete_id)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, res.status().as_u16());
}

#[tokio::test]
async fn delete_non_existent_id() {
    // Arrange
    let address = spawn_app().await;

    let client = reqwest::Client::new();

    let delete_id: Uuid = Faker.fake();

    // Act

    let res = client
        .delete(format!("{}/tasks/{}", address, &delete_id))
        .header("Content-Type", "application/json")
        .json(&delete_id)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, res.status().as_u16());
}
