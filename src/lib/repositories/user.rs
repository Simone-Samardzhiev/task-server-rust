use crate::models::user;
use sqlx::{query, PgPool, Row};
use std::future::Future;

/// `UserRepository` manages user data.
trait UserRepository: Send + Sync + Clone + 'static {
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

    /// `add_user` add the user.
    /// # Error
    /// It can return any error related to database connection.
    fn add_user(&self, user: &user::User) -> impl Future<Output = Result<(), sqlx::Error>> + Send;
}

/// `PostgresUserRepository` is implementation of `UserRepository` with postgres
#[derive(Clone)]
struct PostgresUserRepository {
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
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("SELECT COUNT(*) FROM users WHERE email = $1 OR username = $2")
            .bind(&email)
            .bind(&username)
            .fetch_one(&self.db)
            .await?;

        let count = result.try_get::<i64, _>(0)?;
        Ok(count == 0)
    }

    async fn add_user(&self, user: &user::UserPayload) -> Result<(), sqlx::Error> {
        query("INSERT INTO users(email, username, password) VALUES ($1, $2, $3)")
            .bind(&user.email)
            .bind(&user.username)
            .bind(&user.password)
            .execute(&self.db)
    }
}
