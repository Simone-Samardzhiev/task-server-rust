use crate::auth;
use crate::models::task::{Task, TaskPayload};
use crate::server::TaskState;
use crate::services::task::TaskService;
use crate::utils::api_error_response::APIResult;
use axum::extract::State;
use axum::{Extension, Json};

pub async fn add_task<T: TaskService>(
    State(app): State<TaskState<T>>,
    Extension(claims): Extension<auth::AccessClaims>,
    Json(task): Json<TaskPayload>,
) -> APIResult<Json<Task>> {
    let task = app.task_service.add_task(&task, claims).await?;
    Ok(Json(task))
}
