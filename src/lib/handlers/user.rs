use crate::auth;
use crate::models::token_group::TokenGroup;
use crate::models::user::UserPayload;
use crate::server::UserState;
use crate::services::user::UserService;
use crate::utils::api_error_response::APIResult;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Extension, Json};

pub async fn register<T: UserService>(
    State(app): State<UserState<T>>,
    Json(mut user): Json<UserPayload>,
) -> APIResult<StatusCode> {
    if let Some(error) = user.validate() {
        return Err(error);
    }
    app.user_service.register(&mut user).await
}

pub async fn login<T: UserService>(
    State(app): State<UserState<T>>,
    Json(user): Json<UserPayload>,
) -> APIResult<Json<TokenGroup>> {
    let group = app.user_service.login(&user).await?;
    Ok(Json(group))
}

pub async fn refresh<T: UserService>(
    State(app): State<UserState<T>>,
    Extension(claims): Extension<auth::RefreshClaims>,
) -> APIResult<Json<TokenGroup>> {
    let group = app.user_service.refresh(claims).await?;
    Ok(Json(group))
}
