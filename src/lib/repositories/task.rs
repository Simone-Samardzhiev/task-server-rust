use crate::models::task::Task;
use sqlx::error::Error as SQLXError;
use sqlx::query;
use sqlx::PgPool;
use std::future::Future;

/// Repository that will manage tasks data.
pub trait TaskRepository: Send + Sync + Clone + 'static {
    /// Method that will add a new task and link to a user.
    fn add_task(&self, task: &Task, user_id: i32) -> impl Future<Output=Result<(), SQLXError>> + Send;
}


/// Repository that implements `TaskRepository` using postgres.
#[derive(Clone)]
pub struct PostgresTaskRepository {
    db: PgPool,
}

impl PostgresTaskRepository {
    fn new(db: PgPool) -> Self {
        Self { db }
    }
}

impl TaskRepository for PostgresTaskRepository {
    async fn add_task(&self, task: &Task, user_id: i32) -> Result<(), SQLXError> {
         query("INSERT INTO tasks (id, name, description, priority, date, user_id) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(&task.id)
            .bind(&task.name)
            .bind(&task.description)
            .bind(&task.priority)
            .bind(&task.data)
            .bind(&user_id)
            .execute(&self.db)
            .await?;

        Ok(())
    }
}
