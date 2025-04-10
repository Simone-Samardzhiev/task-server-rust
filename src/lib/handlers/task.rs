use crate::auth::AccessClaims;
use crate::models::task::{Task, TaskPayload};
use crate::server::TaskState;
use crate::services::task::TaskService;
use crate::utils::api_error_response::APIResult;
use axum::extract::State;
use axum::{Extension, Json};

pub async fn add_task<T: TaskService>(
    Json(task): Json<TaskPayload>,
    Extension(claims): Extension<AccessClaims>,
    State(app): State<TaskState<T>>,
) -> APIResult<Json<Task>> {
    let task = app.task_service.add_task(&task, claims).await?;
    Ok(Json(task))
}
