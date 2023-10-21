use askama::Template;

use crate::dtos::TaskDto;

#[derive(Template, Default)]
#[template(path = "tasks/list.html")]
pub struct TaskList {
    pub search_key: Option<String>,
    pub tasks: Vec<TaskDto>,
    pub success_flash_messages: Vec<String>,
    pub error_flash_messages: Vec<String>,
}
