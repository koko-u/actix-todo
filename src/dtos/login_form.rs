use serde::Deserialize;
use serde::Serialize;

use crate::templates::ExtractFieldError;
use crate::templates::Field;
use crate::templates::LoginTemplate;

use super::into_template::IntoTemplate;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, validify::Validify)]
pub struct LoginForm {
    #[modify(trim)]
    #[validate(length(min = 1, message = "Email Address is required."))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is required."))]
    pub password: String,
}

impl IntoTemplate for LoginForm {
    type Template = LoginTemplate;

    fn into_template(self, validation_errors: validify::ValidationErrors) -> Self::Template {
        LoginTemplate {
            email: Field {
                value: self.email,
                error: validation_errors.extract("email"),
            },
            password: Field {
                value: self.password,
                error: validation_errors.extract("password"),
            },
            ..Default::default()
        }
    }
}
