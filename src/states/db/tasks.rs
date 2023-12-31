use async_trait::async_trait;
use error_stack::ResultExt;

use crate::dtos::NewTask;
use crate::dtos::TaskFilter;
use crate::dtos::UpdateTask;
use crate::errors::AppError;
use crate::errors::AppResult;
use crate::models::TaskModel;
use crate::states::repositories::TasksRepository;
use crate::utils::AsOption;

use super::DbState;

#[async_trait]
impl TasksRepository for DbState {
    async fn get_filtered_tasks(
        &self,
        user_id: i64,
        filter: &TaskFilter,
    ) -> AppResult<Vec<TaskModel>> {
        let tasks = sqlx::query_file_as!(
            TaskModel,
            "sql/get_filtered_tasks.sql",
            filter.summary,
            &filter.status_ids,
            filter.status_ids.is_empty(),
            user_id,
        )
        .fetch_all(&self.0)
        .await
        .change_context(AppError)?;

        Ok(tasks)
    }
    async fn get_task_by_id(&self, id: i64) -> AppResult<Option<TaskModel>> {
        let task = sqlx::query_file_as!(TaskModel, "sql/get_task_by_id.sql", id)
            .fetch_optional(&self.0)
            .await
            .change_context(AppError)?;

        Ok(task)
    }
    async fn create_task(&self, user_id: i64, new_task: &NewTask) -> AppResult<TaskModel> {
        let summary = new_task.summary.as_str().empty_as_none();
        let description = new_task.description.as_str().empty_as_none();
        let status_id = new_task.status_id;

        let task: TaskModel;
        {
            let mut tx = self.0.begin().await.change_context(AppError)?;

            task = sqlx::query_file_as!(
                TaskModel,
                "sql/insert_task.sql",
                summary,
                description,
                status_id,
                user_id,
            )
            .fetch_one(tx.as_mut())
            .await
            .change_context(AppError)?;

            tx.commit().await.change_context(AppError)?;
        }

        Ok(task)
    }
    async fn update_task(&self, id: i64, update_task: &UpdateTask) -> AppResult<Option<TaskModel>> {
        let summary = (&update_task.summary).empty_as_none();
        let description = update_task.description.as_str().empty_as_none();
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
    async fn delete_task(&self, id: i64) -> AppResult<Option<TaskModel>> {
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
