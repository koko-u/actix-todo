use askama::Template;

use crate::dtos::TaskDto;
use crate::dtos::TaskFilter;
use crate::models::StatusModel;
use crate::models::UserModel;

#[derive(Template, Default)]
#[template(path = "tasks/list.html")]
pub struct TaskList {
    pub login_user: Option<UserModel>,
    /// current query parametr
    pub task_filter: TaskFilter,
    /// task list to display
    pub tasks: Vec<TaskDto>,
    /// all status for checkbox
    pub statuses: Vec<StatusModel>,
    /// success flash messages
    pub success_flash_messages: Vec<String>,
    /// failure flash messages
    pub error_flash_messages: Vec<String>,
}
