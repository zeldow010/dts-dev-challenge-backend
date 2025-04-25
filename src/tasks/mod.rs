mod error;

use serde::{Deserialize, Serialize};
use sqlx::{self, PgPool};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub use self::error::{Error, Result};

#[derive(Debug, Deserialize)]
pub struct TaskPayload {
    title: String,
    description: Option<String>,
    due: DateTime<Utc>,
}

#[derive(Debug, sqlx::Type, Serialize)]
#[sqlx(type_name = "task_status", rename_all="lowercase")]
enum TaskStatus {
    Draft,
    ToDo,
    Completed
}

#[derive(Debug, Serialize)]
pub struct Task {
    task_id: Option<Uuid>,
    title: String,
    description: Option<String>,
    task_status: TaskStatus,
    due: DateTime<Utc>,
}

impl Task {
    pub fn parse(
        payload: TaskPayload
    ) -> Result<Task> {
        if payload.title.len() <= 3 {
            return Err(Error::ValueTooShort { 
                parameter: String::from("title"), 
                real_length: payload.title.len(), 
                max_length: 3 
            });
        }

        Ok(Task {
            task_id: None,
            title: payload.title,
            description: payload.description,
            due: payload.due,
            task_status: TaskStatus::Draft
        })
    }
}

pub async fn get_all_tasks(
    db: PgPool
) -> Result<Vec<Task>> {
    let result: Vec<Task> = sqlx::query_as!(
        Task,
        r#"SELECT
            task_id,
            title,
            description,
            task_status AS "task_status!: TaskStatus",
            due
        FROM tasks"#r
    )
    .fetch_all(&db)
    .await?;

    Ok(result)
}

pub async fn get_task(
    db: PgPool,
    task_id: Uuid
) -> Result<Task> {
    let result: Task = sqlx::query_as!(
        Task,
        r#"SELECT
            task_id,
            title,
            description,
            task_status AS "task_status!: TaskStatus",
            due
        FROM tasks
        WHERE task_id = $1
        "#r,
        task_id
    ).fetch_optional(&db)
    .await?
    .ok_or(Error::TaskNotFound)?;

    Ok(result)
}

pub async fn create_task(
    db: PgPool,
    new_task: Task,
) -> Result<Uuid> {
    let new_id: Uuid = sqlx::query!(
        r#"
            INSERT INTO tasks (
                title, 
                description, 
                task_status,
                due
            ) VALUES ($1, $2, $3, $4)
            RETURNING task_id
        "#r,
        new_task.title,
        new_task.description,
        new_task.task_status as TaskStatus,
        new_task.due
    )
    .fetch_one(&db)
    .await?
    .task_id;

    Ok(new_id)
}

pub async fn delete_task(
    db: PgPool,
    task_id: Uuid
) -> Result<()> {
    sqlx::query!(
        r#"
            DELETE FROM tasks WHERE task_id = $1
        "#r,
        task_id
    ).execute(&db)
    .await?;

    Ok(())
}