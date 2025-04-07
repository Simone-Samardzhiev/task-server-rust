use crate::models::user::UserPayload;
use crate::repositories::user::UserRepository;
use crate::utils::api_error_response::{APIResult, INTERNAL_SERVER_ERROR_RESPONSE};
use axum::http::StatusCode;
use std::future::Future;
use std::sync::Arc;

trait UserService: Send + Sync + 'static {
    fn register(
        &self,
        user: &mut UserPayload,
    ) -> impl Future<Output = APIResult<StatusCode>> + Send;
}

pub struct DefaultUserService<T: UserRepository> {
    repository: Arc<T>,
}

impl<T: UserRepository> DefaultUserService<T> {
    pub fn new(repository: T) -> Self {
        repository
    }
}

impl<T: UserRepository> UserService for DefaultUserService<T> {
    async fn register(
        &self,
        user: &mut UserPayload,
    ) -> impl Future<Output = APIResult<StatusCode>> + Send {
        self.repository
            .check_user_email_username(&user.email, &user.username)
            .await?;

        user.password = bcrypt::hash(&user.password, bcrypt::DEFAULT_COST)
            .map_err(INTERNAL_SERVER_ERROR_RESPONSE)?;

        self.repository.add_user(&user).await
    }
}
