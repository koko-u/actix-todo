use async_trait::async_trait;

use crate::dtos::NewTask;
use crate::dtos::NewUser;
use crate::dtos::TaskFilter;
use crate::dtos::UpdateTask;
use crate::errors::AppResult;
use crate::models::StatusModel;
use crate::models::TaskModel;
use crate::models::UserModel;

pub trait DbRepository: TasksRepository + StatusRepository + UsersRepository {}

#[async_trait]
pub trait TasksRepository {
    /// get task using filter conditions
    async fn get_filtered_tasks(&self, user_id: i64, filter: &TaskFilter) -> AppResult<Vec<TaskModel>>;

    /// get task by id
    async fn get_task_by_id(&self, id: i64) -> AppResult<Option<TaskModel>>;

    /// create new task
    async fn create_task(&self, user_id: i64, new_task: &NewTask) -> AppResult<TaskModel>;

    /// update the task with id
    async fn update_task(&self, id: i64, update_task: &UpdateTask) -> AppResult<Option<TaskModel>>;

    /// delete the task with id
    async fn delete_task(&self, id: i64) -> AppResult<Option<TaskModel>>;
}

#[async_trait]
pub trait StatusRepository {
    /// get all statuses
    async fn get_all_statuses(&self) -> AppResult<Vec<StatusModel>>;

    /// get status by id
    async fn get_status_by_id(&self, status_id: i64) -> AppResult<Option<StatusModel>>;
}

#[async_trait]
pub trait UsersRepository {
    /// create new user
    async fn create_user(&self, new_user: &NewUser) -> AppResult<UserModel>;

    /// authentication
    async fn auth_user(&self, email: &str, password: &str) -> AppResult<Option<UserModel>>;

    /// get user by id
    async fn get_user_by_id(&self, id: i64) -> AppResult<Option<UserModel>>;
}
