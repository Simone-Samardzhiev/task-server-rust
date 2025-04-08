use crate::models::user::UserPayload;
use crate::repositories::user::UserRepository;
use crate::utils::api_error_response::{APIErrorResponse, APIResult};
use axum::http::StatusCode;
use std::future::Future;
use std::sync::Arc;

pub trait UserService: Send + Sync + Clone + 'static {
    fn register(
        &self,
        user: &mut UserPayload,
    ) -> impl Future<Output = APIResult<StatusCode>> + Send;
}
#[derive(Clone)]
pub struct DefaultUserService<T: UserRepository> {
    repository: Arc<T>,
}

impl<T: UserRepository> DefaultUserService<T> {
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }
}

impl<T: UserRepository> UserService for DefaultUserService<T> {
    async fn register(&self, user: &mut UserPayload) -> APIResult<StatusCode> {
        if !self
            .repository
            .check_user_email_username(&user.email, &user.username)
            .await?
        {
            return Err(APIErrorResponse::new(
                StatusCode::CONFLICT,
                String::from("Email or username already exists"),
            ));
        }

        user.password = bcrypt::hash(&user.password, bcrypt::DEFAULT_COST).map_err(|err| {
            APIErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        })?;

        self.repository.add_user(&user).await?;
        Ok(StatusCode::CREATED)
    }
}
