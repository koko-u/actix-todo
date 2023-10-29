use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct Field<T> {
    pub value: T,
    pub error: FieldError,
}
impl<T> From<T> for Field<T> {
    fn from(value: T) -> Self {
        Self {
            value,
            error: FieldError::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct FieldError {
    pub is_invalid: &'static str,
    pub message: String,
}
impl FieldError {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            is_invalid: "is-invalid",
            message: message.into(),
        }
    }
}

pub trait ExtractFieldError {
    fn extract(&self, name: &'static str) -> FieldError;
}
impl ExtractFieldError for validify::ValidationErrors {
    fn extract(&self, name: &'static str) -> FieldError {
        self.errors()
            .iter()
            .find(|e| e.field_name().is_some_and(|fname| fname == name))
            .map(|validation_error| FieldError {
                is_invalid: "is-invalid",
                message: validation_error.message().unwrap_or_default(),
            })
            .unwrap_or_default()
    }
}
