use error_stack::ResultExt;

use crate::dtos::NewTask;
use crate::dtos::UpdateTask;
use crate::errors::AppError;
use crate::errors::AppResult;
use crate::models::TaskModel;

use super::DbState;

impl DbState {
    pub async fn get_all_tasks(&self) -> AppResult<Vec<TaskModel>> {
        let tasks = sqlx::query_file_as!(TaskModel, "sql/get_all_tasks.sql")
            .fetch_all(&self.0)
            .await
            .change_context(AppError)?;

        Ok(tasks)
    }
    pub async fn get_task_by_id(&self, id: i64) -> AppResult<Option<TaskModel>> {
        let task = sqlx::query_file_as!(TaskModel, "sql/get_task_by_id.sql", id)
            .fetch_optional(&self.0)
            .await
            .change_context(AppError)?;

        Ok(task)
    }
    pub async fn save_task(&self, new_task: &NewTask) -> AppResult<TaskModel> {
        let summary = new_task.summary.into_option();
        let description = new_task.description.into_option();
        let status_id = new_task.status_id;

        let task: TaskModel;
        {
            let mut tx = self.0.begin().await.change_context(AppError)?;

            task = sqlx::query_file_as!(
                TaskModel,
                "sql/insert_task.sql",
                summary,
                description,
                status_id
            )
            .fetch_one(tx.as_mut())
            .await
            .change_context(AppError)?;

            tx.commit().await.change_context(AppError)?;
        }

        Ok(task)
    }
    pub async fn update_task(
        &self,
        id: i64,
        update_task: &UpdateTask,
    ) -> AppResult<Option<TaskModel>> {
        let summary = update_task.summary.into_option();
        let description = update_task.description.into_option();
        let status_id = update_task.status_id;

        let task: Option<TaskModel>;
        {
            let mut tx = self.0.begin().await.change_context(AppError)?;

            task = sqlx::query_file_as!(
                TaskModel,
                "sql/update_task_by_id.sql",
                id,
                summary,
                description,
                status_id
            )
            .fetch_optional(tx.as_mut())
            .await
            .change_context(AppError)?;

            tx.commit().await.change_context(AppError)?;
        }
        Ok(task)
    }
    pub async fn delete_task(&self, id: i64) -> AppResult<Option<TaskModel>> {
        let task: Option<TaskModel>;
        {
            let mut tx = self.0.begin().await.change_context(AppError)?;

            task = sqlx::query_file_as!(TaskModel, "sql/delete_task_by_id.sql", id)
                .fetch_optional(tx.as_mut())
                .await
                .change_context(AppError)?;

            tx.commit().await.change_context(AppError)?;
        }

        Ok(task)
    }
}

trait AsOption: Into<String> {
    fn into_option(self) -> Option<String>;
}
impl<'a> AsOption for &'a str {
    fn into_option(self) -> Option<String> {
        let s: String = self.into();
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }
}
