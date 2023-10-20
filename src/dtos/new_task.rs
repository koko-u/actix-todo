use serde::Deserialize;
use serde::Serialize;

use crate::templates::ExtractFieldError;
use crate::templates::Field;
use crate::templates::NewTaskTemplate;

use super::IsValidStatus;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize, validify::Validify)]
pub struct NewTask {
    #[modify(trim)]
    #[validate(length(min = 1, message = "Summary field is required."))]
    #[validate(length(max = 255, message = "Summary field length is upto 255 characters"))]
    pub summary: String,
    #[validate(length(max = 1000, message = "Description is too long."))]
    pub description: String,
    #[validate(range(min = 1.0, message = "status is required"))]
    pub status_id: i64,
}

impl NewTask {
    pub fn into_template(self, validation_errors: validify::ValidationErrors) -> NewTaskTemplate {
        NewTaskTemplate {
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

impl IsValidStatus for NewTask {
    fn status_id(&self) -> i64 {
        self.status_id
    }
}
