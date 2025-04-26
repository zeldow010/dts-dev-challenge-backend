use std::time::Duration;

use axum::{
    extract::{Path, State}, http::StatusCode, middleware, routing::{get, patch}, Json, Router
};
use error::Result;
use response_mapper::mw_response_map;
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

mod error;
mod response_mapper;
pub mod tasks;

pub async fn app() -> Router {
    let db = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect("postgres://app:secret@localhost:5432/dts_tasks")
        .await
        .unwrap();

    Router::new()
        .route("/health_check", get(health_check))
        .route("/tasks", get(get_all_tasks).post(create_task))
        .route(
            "/tasks/{task_id}",
            get(get_specific_task).delete(delete_task),
        )
        .route("/tasks/{task_id}/next_status", patch(update_task_status))
        .with_state(db)
        .layer(middleware::map_response(mw_response_map))
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn get_all_tasks(State(db): State<PgPool>) -> Result<Json<Vec<tasks::Task>>> {
    let tasks = tasks::get_all_tasks(db).await?;

    Ok(Json(tasks))
}

async fn get_specific_task(
    State(db): State<PgPool>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<tasks::Task>> {
    let task = tasks::get_task(db, task_id).await?;

    Ok(Json(task))
}

async fn create_task(
    State(db): State<PgPool>,
    Json(payload): Json<tasks::TaskPayload>,
) -> Result<Json<Uuid>> {
    let new_task: tasks::Task = tasks::Task::parse(payload)?;

    let new_id = tasks::create_task(db, new_task).await?;

    Ok(Json(new_id))
}

async fn update_task_status(
    State(db): State<PgPool>,
    Path(task_id): Path<Uuid>
) -> Result<Json<tasks::TaskStatus>> {
    let task_status = tasks::update_task_status(db, task_id).await?;

    Ok(Json(task_status))
}

async fn delete_task(State(db): State<PgPool>, Path(task_id): Path<Uuid>) -> StatusCode {
    let delete_res = tasks::delete_task(db, task_id).await;

    match delete_res {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
