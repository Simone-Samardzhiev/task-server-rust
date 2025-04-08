use crate::models::user;
use crate::models::user::User;
use sqlx::{query, Error as SQLError, PgPool, Row};
use std::future::Future;

/// `UserRepository` manages user data.
pub trait UserRepository: Send + Sync + Clone + 'static {
    /// `check_user_email_username` check if the user email and username are unique
    /// # Error
    /// It can return any error related to database connection.
    ///
    /// # Returned value
    /// `true` if the both the email and username are unique otherwise `false`.
    fn check_user_email_username(
        &self,
        email: &str,
        username: &str,
    ) -> impl Future<Output = Result<bool, sqlx::Error>> + Send;

    /// `add_user` adds the user.
    /// # Error
    /// It can return any error related to database connection.
    fn add_user(
        &self,
        user: &user::UserPayload,
    ) -> impl Future<Output = Result<(), sqlx::Error>> + Send;

    /// `get_user_by_email` will fetch user with specified email.
    ///
    /// # Error
    /// It can return any error related to database connection.
    ///
    /// # Returns
    /// `Ok(User)` If the user is found.
    fn get_user_by_email(
        &self,
        email: &str,
    ) -> impl Future<Output = Result<user::User, sqlx::Error>> + Send;
}

/// `PostgresUserRepository` is implementation of `UserRepository` with postgres
#[derive(Clone)]
pub struct PostgresUserRepository {
    db: PgPool,
}

impl PostgresUserRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

impl UserRepository for PostgresUserRepository {
    async fn check_user_email_username(
        &self,
        email: &str,
        username: &str,
    ) -> Result<bool, SQLError> {
        let result = sqlx::query("SELECT COUNT(*) FROM users WHERE email = $1 OR username = $2")
            .bind(&email)
            .bind(&username)
            .fetch_one(&self.db)
            .await?;

        let count: i64 = result.try_get(0)?;
        Ok(count == 0)
    }

    async fn add_user(&self, user: &user::UserPayload) -> Result<(), SQLError> {
        query("INSERT INTO users(email, username, password) VALUES ($1, $2, $3)")
            .bind(&user.email)
            .bind(&user.username)
            .bind(&user.password)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn get_user_by_email(&self, email: &str) -> Result<User, SQLError> {
        let result = query("SELECT id, email, username, password FROM USERS WHERE email = $1")
            .bind(&email)
            .fetch_one(&self.db)
            .await?;

        let id: i64 = result.try_get(0)?;
        let email: String = result.try_get(1)?;
        let username: String = result.try_get(2)?;
        let password: String = result.try_get(3)?;

        Ok(User::new(id, email, username, password))
    }
}
