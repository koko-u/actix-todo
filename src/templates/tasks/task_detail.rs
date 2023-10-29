use askama::Template;

use crate::dtos::TaskDto;
use crate::models::UserModel;

#[derive(Template)]
#[template(path = "tasks/detail.html")]
pub struct TaskDetail {
    pub login_user: Option<UserModel>,
    pub task: TaskDto,
}
