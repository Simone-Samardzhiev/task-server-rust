use crate::utils::api_error_response::APIErrorResponse;
use axum::http::StatusCode;
use chrono::Utc;
use uuid::Uuid;

/// Struct holding task data.
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub priority: String,
    pub data: chrono::DateTime<Utc>,
}

impl Task {
    pub fn new(
        id: Uuid,
        name: String,
        description: String,
        priority: String,
        data: chrono::DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            priority,
            data,
        }
    }

    pub fn validate(&self) -> Option<APIErrorResponse> {
        if self.name.is_empty() {
            return Some(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Name cannot be empty"),
            ));
        }

        if self.description.is_empty() {
            return Some(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Description cannot be empty"),
            ));
        }

        if self.priority.is_empty() {
            return Some(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Priority cannot be empty"),
            ));
        }

        None
    }
}

/// Struct holding new task data.
pub struct TaskPayload {
    pub name: String,
    pub description: String,
    pub priority: String,
    pub data: chrono::DateTime<Utc>,
}

impl TaskPayload {
    pub fn new(
        name: String,
        description: String,
        priority: String,
        data: chrono::DateTime<Utc>,
    ) -> Self {
        Self {
            name,
            description,
            priority,
            data,
        }
    }

    pub fn validate(&self) -> Option<APIErrorResponse> {
        if self.name.is_empty() {
            return Some(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Name cannot be empty"),
            ));
        }

        if self.description.is_empty() {
            return Some(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Description cannot be empty"),
            ));
        }

        if self.priority.is_empty() {
            return Some(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Priority cannot be empty"),
            ));
        }

        None
    }
}
