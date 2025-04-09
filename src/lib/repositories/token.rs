use chrono::NaiveDateTime;
use sqlx::{query, Error as SQLError, Executor, PgPool};
use std::future::Future;
use uuid::Uuid;

/// `TokenRepository` manages token data.
pub trait TokenRepository: Send + Sync + Clone + 'static {
    /// Method that will add a refresh token.
    ///
    /// # Errors
    /// It can return any error related to database connection.
    fn add_token(
        &self,
        id: Uuid,
        exp: NaiveDateTime,
        user_id: i32,
    ) -> impl Future<Output = Result<(), SQLError>> + Send;
}

/// `PostgresTokenRepository` is implementation of `TaskRepository` with postgres
#[derive(Clone)]
pub struct PostgresTokenRepository {
    db: PgPool,
}

impl PostgresTokenRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

impl TokenRepository for PostgresTokenRepository {
    async fn add_token(&self, id: Uuid, exp: NaiveDateTime, user_id: i32) -> Result<(), SQLError> {
        query("INSERT INTO tokens (id, exp, user_id) VALUES ($1, $2, $3)")
            .bind(id)
            .bind(exp)
            .bind(user_id)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
