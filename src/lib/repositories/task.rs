use crate::models::task::Task;
use chrono::{DateTime, Utc};
use sqlx::error::Error as SQLXError;
use sqlx::PgPool;
use sqlx::{query, Row};
use std::future::Future;
use uuid::Uuid;

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

    /// Method used to get all tasks linked to a user.
    fn get_tasks_by_user_id(
        &self,
        user_id: i32,
    ) -> impl Future<Output = Result<Vec<Task>, sqlx::Error>> + Send;

    /// Method that will update existing task.
    fn update_task(&self, task: &Task) -> impl Future<Output = Result<bool, SQLXError>> + Send;
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

    async fn get_tasks_by_user_id(&self, user_id: i32) -> Result<Vec<Task>, SQLXError> {
        let rows =
            query("SELECT id, name, description, priority, date FROM tasks WHERE user_id = $1")
                .bind(user_id)
                .fetch_all(&self.db)
                .await?;

        let mut result: Vec<Task> = Vec::with_capacity(rows.len());

        for row in rows {
            let id: Uuid = row.try_get(0)?;
            let name: String = row.try_get(1)?;
            let description: String = row.try_get(2)?;
            let priority: String = row.try_get(3)?;
            let date: DateTime<Utc> = row.try_get(4)?;
            result.push(Task::new(id, name, description, priority, date))
        }

        Ok(result)
    }

    async fn update_task(&self, task: &Task) -> Result<bool, SQLXError> {
        let result = query(
            "UPDATE tasks SET name = $2, description = $3, priority = $4, date = $5 WHERE id = $6",
        )
        .bind(&task.name)
        .bind(&task.description)
        .bind(&task.priority)
        .bind(&task.date)
        .execute(&self.db)?;

        Ok(result > 0)
    }
}
