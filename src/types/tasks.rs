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

#[derive(Debug, Deserialize)]
pub struct TaskPayload {
    pub name: String,
    pub description: String,
    pub r#type: i32,
    pub due_date: DateTime<Utc>,
}
