use crate::models::token_group::TokenGroup;
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
    if let Some(error) = user.validate() {
        return Err(error);
    }
    app.user_service.register(&mut user).await
}

pub async fn login<T: UserService>(
    State(app): State<AppState<T>>,
    Json(user): Json<UserPayload>,
) -> APIResult<TokenGroup> {
    app.user_service.login(&user).await
}
