use crate::auth::AccessClaims;
use crate::models::task::{Task, TaskPayload};
use crate::repositories::task::TaskRepository;
use crate::utils::api_error_response::{APIErrorResponse, APIResult};
use axum::http::StatusCode;
use std::future::Future;
use std::sync::Arc;
use uuid::Uuid;

/// Service used to manage task business logic.
pub trait TaskService: Send + Sync + Clone + 'static {
    /// Method that will add a new task.
    /// # Returns
    /// The newly created task.
    fn add_task(
        &self,
        task: &TaskPayload,
        claims: AccessClaims,
    ) -> impl Future<Output = APIResult<Task>> + Send;

    /// Method that will fetch all task of a user.
    /// # Returns
    /// Vector will all tasks.
    fn get_task(&self, claims: AccessClaims) -> impl Future<Output = APIResult<Vec<Task>>> + Send;

    /// Method that will update an existing task.
    fn update_task(&self, task: &Task) -> impl Future<Output = APIResult<()>> + Send;

    /// Method that will delete task by id.
    fn delete_task(&self, id: Uuid) -> impl Future<Output = APIResult<()>> + Send;
}

#[derive(Clone)]
pub struct DefaultTaskService<T: TaskRepository> {
    repository: Arc<T>,
}

impl<T: TaskRepository> DefaultTaskService<T> {
    pub fn new(repository: Arc<T>) -> Self {
        Self { repository }
    }
}

impl<T: TaskRepository> TaskService for DefaultTaskService<T> {
    async fn add_task(&self, task: &TaskPayload, claims: AccessClaims) -> APIResult<Task> {
        let result = self.repository.check_priority(&task.priority).await?;
        if !result {
            return Err(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Invalid priority"),
            ));
        }

        let id = Uuid::new_v4();
        let task = Task::new(
            id,
            task.name.clone(),
            task.description.clone(),
            task.priority.clone(),
            task.date,
        );

        self.repository.add_task(&task, claims.sub).await?;

        Ok(task)
    }

    async fn get_task(&self, claims: AccessClaims) -> APIResult<Vec<Task>> {
        let tasks = self.repository.get_tasks_by_user_id(claims.sub).await?;
        Ok(tasks)
    }

    async fn update_task(&self, task: &Task) -> APIResult<()> {
        if !self.repository.check_priority(&task.priority).await? {
            return Err(APIErrorResponse::new(
                StatusCode::BAD_REQUEST,
                String::from("Invalid priority"),
            ));
        }

        if !self.repository.update_task(&task).await? {
            return Err(APIErrorResponse::new(
                StatusCode::NOT_FOUND,
                String::from("Task not found"),
            ));
        }

        Ok(())
    }

    async fn delete_task(&self, id: Uuid) -> APIResult<()> {
        if self.repository.delete_task_by_id(id).await? {
            return Ok(());
        }

        Err(APIErrorResponse::new(
            StatusCode::NOT_FOUND,
            String::from("Task not found"),
        ))
    }
}
