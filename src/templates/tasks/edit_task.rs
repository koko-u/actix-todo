use askama::Template;

use crate::models::StatusModel;
use crate::templates::Field;

#[derive(Template, Default)]
#[template(path = "tasks/edit.html")]
pub struct EditTaskTemplate {
    pub statuses: Vec<StatusModel>,
    pub id: i64,
    pub summary: Field<String>,
    pub description: Field<String>,
    pub status: Field<i64>,
}


