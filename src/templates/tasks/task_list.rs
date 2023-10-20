use askama::Template;

use crate::dtos::TaskDto;

#[derive(Template)]
#[template(path = "tasks/list.html")]
pub struct TaskList {
    pub tasks: Vec<TaskDto>,
    pub info_flash_messages: Vec<String>,
    pub error_flash_messages: Vec<String>,
}
