use askama::Template;

use crate::dtos::TaskDto;

#[derive(Template)]
#[template(path = "tasks/detail.html")]
pub struct TaskDetail {
    pub task: TaskDto,
}
