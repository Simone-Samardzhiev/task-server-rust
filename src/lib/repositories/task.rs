use crate::models::task::Task;
use sqlx::error::Error as SQLXError;
use sqlx::PgPool;
use sqlx::{query, Row};
use std::future::Future;

/// Repository that will manage tasks data.
pub trait TaskRepository: Send + Sync + Clone + 'static {
    /// Method that will add a new task and link to a user.
    fn add_task(
        &self,
        task: &Task,
        user_id: i32,
    ) -> impl Future<Output = Result<(), SQLXError>> + Send;

    /// Method used to check whether a priority is valid.
    fn check_priority(
        &self,
        priority: &str,
    ) -> impl Future<Output = Result<bool, sqlx::Error>> + Send;
}

/// Repository that implements `TaskRepository` using postgres.
#[derive(Clone)]
pub struct PostgresTaskRepository {
    db: PgPool,
}

impl PostgresTaskRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

impl TaskRepository for PostgresTaskRepository {
    async fn add_task(&self, task: &Task, user_id: i32) -> Result<(), SQLXError> {
        println!("Executing query for adding task with task {:?}", task);

        query("INSERT INTO tasks (id, name, description, priority, date, user_id) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(&task.id)
            .bind(&task.name)
            .bind(&task.description)
            .bind(&task.priority)
            .bind(&task.date)
            .bind(&user_id)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    async fn check_priority(&self, priority: &str) -> Result<bool, SQLXError> {
        let row = query("SELECT COUNT(*) FROM priorities WHERE priority = $1")
            .bind(priority)
            .fetch_one(&self.db)
            .await?;

        let count: i64 = row.get(0);

        Ok(count > 0)
    }
}
