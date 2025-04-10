use crate::auth::AccessClaims;
use crate::models::task::{Task, TaskPayload};
use crate::repositories::task::TaskRepository;
use crate::utils::api_error_response::APIResult;
use std::future::Future;
use std::sync::Arc;
use uuid::Uuid;

pub trait TaskService: Send + Sync + Clone + 'static {
    fn add_task(task: &TaskPayload, claims: AccessClaims) -> impl Future<Output = APIResult<Task>>;
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
    fn add_task(task: &TaskPayload, claims: AccessClaims) -> APIResult<Task> {
        let id = Uuid::new_v4();
        let task = Task::new(
            id,
            task.name.clone(),
            task.priority.clone(),
            task.description.clone(),
            task.date,
        );
    }
}
