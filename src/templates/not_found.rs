use askama::Template;

use crate::models::UserModel;

#[derive(Template, Default)]
#[template(path = "notfound.html")]
pub struct NotFoundTemplate {
    pub login_user: Option<UserModel>,
}
