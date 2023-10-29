use askama::Template;

use crate::models::UserModel;

#[derive(Template, Default)]
#[template(path = "index.html")]
pub struct RootTemplate {
    pub login_user: Option<UserModel>,
}
