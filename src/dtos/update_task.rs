use serde::Deserialize;
use serde::Serialize;

use crate::templates::EditTaskTemplate;
use crate::templates::ExtractFieldError;
use crate::templates::Field;

use super::into_template::IntoTemplate;
use super::IsValidStatus;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, validify::Validify)]
pub struct UpdateTask {
    #[modify(trim)]
    #[validate(length(min = 1, message = "Summary field is required."))]
    #[validate(length(max = 255, message = "Summary field length is upto 255 characters"))]
    pub summary: String,

    #[validate(length(max = 1000, message = "Description is too long."))]
    pub description: String,

    #[validate(range(min = 1.0, message = "status is required"))]
    pub status_id: i64,
}

impl IntoTemplate for UpdateTask {
    type Template = EditTaskTemplate;

    fn into_template(self, validation_errors: validify::ValidationErrors) -> EditTaskTemplate {
        EditTaskTemplate {
            summary: Field {
                value: self.summary,
                error: validation_errors.extract("summary"),
            },
            description: Field {
                value: self.description,
                error: validation_errors.extract("description"),
            },
            status: Field {
                value: self.status_id,
                error: validation_errors.extract("status_id"),
            },
            ..Default::default()
        }
    }
}

impl IsValidStatus for UpdateTask {
    fn status_id(&self) -> i64 {
        self.status_id
    }
}
