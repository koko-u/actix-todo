use serde::Deserialize;
use serde::Serialize;

use crate::templates::ExtractFieldError;
use crate::templates::Field;
use crate::templates::SignUpTemplate;

use super::IntoTemplate;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, validify::Validify)]
#[validate(validate_password_match)]
pub struct NewUser {
    #[modify(trim)]
    #[validate(length(min = 1, message = "Name field is required."))]
    #[validate(length(max = 255, message = "Name field length is upto 255 characters"))]
    pub name: String,

    #[modify(trim)]
    #[validate(length(min = 1, message = "Email Address field is required."))]
    #[validate(length(
        max = 255,
        message = "Email Address field length is upto 255 characters"
    ))]
    pub email: String,

    #[validate(length(min = 1, message = "Password field is required."))]
    pub password: String,

    #[validate(length(min = 1, message = "Password field is required."))]
    pub password_confirm: String,
}

fn validate_password_match(new_user: &NewUser) -> Result<(), validify::ValidationErrors> {
    if new_user.password.is_empty() {
        return Ok(());
    }
    if new_user.password_confirm.is_empty() {
        return Ok(());
    }

    if new_user.password != new_user.password_confirm {
        // password not match
        let mut errors = validify::ValidationErrors::new();
        errors.add(
            validify::ValidationError::new_schema("password")
                .with_message("Password does not match".into()),
        );
        return Err(errors);
    }

    Ok(())
}
impl IntoTemplate for NewUser {
    type Template = SignUpTemplate;

    fn into_template(self, validation_errors: validify::ValidationErrors) -> Self::Template {
        SignUpTemplate {
            name: Field {
                value: self.name,
                error: validation_errors.extract("name"),
            },
            email: Field {
                value: self.email,
                error: validation_errors.extract("email"),
            },
            password: Field {
                value: self.password,
                error: validation_errors.extract("password"),
            },
            password_confirm: Field {
                value: self.password_confirm,
                error: validation_errors.extract("password"),
            },
            ..Default::default()
        }
    }
}
