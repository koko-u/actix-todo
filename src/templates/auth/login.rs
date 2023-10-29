use askama::Template;

use crate::models::UserModel;
use crate::templates::Field;

#[derive(Template, Default)]
#[template(path = "auth/login.html")]
pub struct LoginTemplate {
    pub email: Field<String>,
    pub password: Field<String>,
    pub login_user: Option<UserModel>,
}
