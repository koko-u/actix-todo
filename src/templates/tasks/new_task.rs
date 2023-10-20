use askama::Template;

use crate::models::StatusModel;
use crate::templates::Field;

#[derive(Template, Default)]
#[template(path = "tasks/new.html")]
pub struct NewTaskTemplate {
    pub statuses: Vec<StatusModel>,
    pub summary: Field<String>,
    pub description: Field<String>,
    pub status: Field<i64>,
}
