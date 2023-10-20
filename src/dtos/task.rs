use serde::Deserialize;
use serde::Serialize;

use crate::models::TaskModel;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaskDto {
    pub id: i64,
    pub summary: String,
    pub description: Option<String>,
    pub status_name: String,
}

impl From<TaskModel> for TaskDto {
    fn from(task: TaskModel) -> Self {
        Self {
            id: task.id,
            summary: task.summary,
            description: task.description,
            status_name: task.status_name,
        }
    }
}
