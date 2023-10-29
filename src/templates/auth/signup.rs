use askama::Template;

use crate::models::UserModel;
use crate::templates::Field;

#[derive(Template, Default)]
#[template(path = "auth/signup.html")]
pub struct SignUpTemplate {
    pub name: Field<String>,
    pub email: Field<String>,
    pub password: Field<String>,
    pub password_confirm: Field<String>,
    pub login_user: Option<UserModel>,
}
