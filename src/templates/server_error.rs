use askama::Template;

use crate::models::UserModel;

#[derive(Template, Default)]
#[template(path = "error.html")]
pub struct ServerErrorTemplate {
    pub error_message: String,
    pub login_user: Option<UserModel>,
}
