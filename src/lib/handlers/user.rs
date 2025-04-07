use crate::models::user::UserPayload;
use crate::server::AppState;
use crate::services::user::UserService;
use crate::utils::api_error_response::APIResult;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;

pub async fn register<T: UserService>(
    State(app): State<AppState<T>>,
    Json(mut user): Json<UserPayload>,
) -> APIResult<StatusCode> {
    app.user_service.register(&mut user).await
}
