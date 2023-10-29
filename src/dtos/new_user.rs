use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, validify::Validify)]
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
}

impl NewUser {
    /*pub fn into_template(self, validation_errors: validify::ValidationErrors) ->  {
        LoginTemplate {
            email: Field {
                value: self.email,
                error: validation_errors.extract("email"),
            },
            password: Field {
                value: self.password
            },
        }
    }
     */
}
