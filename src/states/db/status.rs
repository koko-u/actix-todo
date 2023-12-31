use async_trait::async_trait;
use error_stack::ResultExt;

use crate::errors::AppError;
use crate::errors::AppResult;
use crate::models::StatusModel;
use crate::states::repositories::StatusRepository;


use super::DbState;

#[async_trait]
impl StatusRepository for DbState {
    async fn get_all_statuses(&self) -> AppResult<Vec<StatusModel>> {
        let statuses = sqlx::query_file_as!(StatusModel, "sql/get_all_statuses.sql")
            .fetch_all(&self.0)
            .await
            .change_context(AppError)?;
        Ok(statuses)
    }
    async fn get_status_by_id(&self, status_id: i64) -> AppResult<Option<StatusModel>> {
        let status_row = sqlx::query_as!(
            StatusModel,
            "SELECT id, name FROM status WHERE id = $1",
            status_id
        )
        .fetch_optional(&self.0)
        .await
        .change_context(AppError)?;

        Ok(status_row)
    }
}
