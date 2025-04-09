use crate::auth::Authenticator;
use crate::models::token_group::TokenGroup;
use crate::models::user::UserPayload;
use crate::repositories::token::TokenRepository;
use crate::repositories::user::UserRepository;
use crate::utils::api_error_response::{APIErrorResponse, APIResult};
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use std::future::Future;
use std::ops::Add;
use std::sync::Arc;
use uuid::Uuid;

pub trait UserService: Send + Sync + Clone + 'static {
    fn register(
        &self,
        user: &mut UserPayload,
    ) -> impl Future<Output = APIResult<StatusCode>> + Send;

    fn login(&self, user: &UserPayload) -> impl Future<Output = APIResult<TokenGroup>> + Send;
}
#[derive(Clone)]
pub struct DefaultUserService<U, T>
where
    U: UserRepository,
    T: TokenRepository,
{
    user_repository: Arc<U>,
    token_repository: Arc<T>,
    authenticator: Arc<Authenticator>,
}

impl<U, T> DefaultUserService<U, T>
where
    U: UserRepository,
    T: TokenRepository,
{
    pub fn new(
        user_repository: Arc<U>,
        token_repository: Arc<T>,
        authenticator: Arc<Authenticator>,
    ) -> Self {
        Self {
            user_repository,
            token_repository,
            authenticator,
        }
    }

    async fn create_token_group(&self, user_id: i64) -> APIResult<TokenGroup> {
        let access_token = self
            .authenticator
            .new_access_token(
                user_id,
                Utc::now().add(Duration::minutes(10)).timestamp() as usize,
            )
            .map_err(|err| {
                APIErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            })?;

        let refresh_token_id = Uuid::new_v4();
        let refresh_token_duration = Utc::now() + Duration::days(15);
        let refresh_token = self
            .authenticator
            .new_refresh_token(
                refresh_token_id,
                user_id,
                refresh_token_duration.timestamp() as usize,
            )
            .map_err(|err| {
                APIErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            })?;

        self.token_repository
            .add_token(
                refresh_token_id,
                refresh_token_duration.naive_utc(),
                user_id,
            )
            .await?;

        Ok(TokenGroup::new(access_token, refresh_token))
    }
}

impl<U, T> UserService for DefaultUserService<U, T>
where
    U: UserRepository,
    T: TokenRepository,
{
    async fn register(&self, user: &mut UserPayload) -> APIResult<StatusCode> {
        if !self
            .user_repository
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

        self.user_repository.add_user(&user).await?;
        Ok(StatusCode::CREATED)
    }

    async fn login(&self, user: &UserPayload) -> APIResult<TokenGroup> {
        let fetched_user = self
            .user_repository
            .get_user_by_email(&user.email)
            .await
            .map_err(|err| match err {
                sqlx::Error::RowNotFound => APIErrorResponse::new(
                    StatusCode::UNAUTHORIZED,
                    String::from("Invalid email or password"),
                ),
                _ => APIErrorResponse::from(err),
            })?;

        if !bcrypt::verify(&user.password, &fetched_user.password).is_ok() {
            return Err(APIErrorResponse::new(
                StatusCode::UNAUTHORIZED,
                String::from("Invalid email or password"),
            ));
        }

        self.create_token_group(fetched_user.id).await
    }
}
