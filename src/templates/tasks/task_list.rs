use askama::Template;

use crate::dtos::TaskDto;
use crate::dtos::TaskFilter;
use crate::models::StatusModel;

#[derive(Template, Default)]
#[template(path = "tasks/list.html")]
pub struct TaskList {
    pub task_filter: TaskFilter,
    pub tasks: Vec<TaskDto>,
    pub statuses: Vec<StatusModel>,
    pub success_flash_messages: Vec<String>,
    pub error_flash_messages: Vec<String>,
}
