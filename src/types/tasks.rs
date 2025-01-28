use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Task {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub r#type: i32,
    pub due_date: DateTime<Utc>,
    pub date_completed: Option<DateTime<Utc>>,
    pub date_deleted: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(
        id: uuid::Uuid,
        name: String,
        description: String,
        r#type: i32,
        due_date: DateTime<Utc>,
        date_completed: Option<DateTime<Utc>>,
        date_deleted: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            r#type,
            due_date,
            date_completed,
            date_deleted,
        }
    }

    pub fn new_from_task_payload(payload: &TaskPayload) -> Self {
        Self::new(
            uuid::Uuid::new_v4(),
            payload.name.clone(),
            payload.description.clone(),
            payload.r#type,
            payload.due_date,
            None,
            None,
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct TaskPayload {
    pub name: String,
    pub description: String,
    pub r#type: i32,
    pub due_date: DateTime<Utc>,
}
