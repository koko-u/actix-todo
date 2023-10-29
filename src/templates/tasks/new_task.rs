use askama::Template;

use crate::models::StatusModel;
use crate::models::UserModel;
use crate::templates::Field;

#[derive(Template, Default)]
#[template(path = "tasks/new.html")]
pub struct NewTaskTemplate {
    pub login_user: Option<UserModel>,
    pub statuses: Vec<StatusModel>,
    pub summary: Field<String>,
    pub description: Field<String>,
    pub status: Field<i64>,
}
