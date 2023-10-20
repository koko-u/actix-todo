use crate::models::StatusModel;

pub trait IsValidStatus {
    fn status_id(&self) -> i64;

    fn is_valid_status(&self, statuses: &[StatusModel]) -> validify::ValidationErrors {
        if statuses.iter().any(|st| st.id == self.status_id()) {
            // valid status_id
            validify::ValidationErrors::default()
        } else {
            // in valid status_id
            let mut validation_error =
                validify::ValidationError::new_field("status_id", "status_id");
            validation_error.set_message(format!("Invalid status id: {}", self.status_id()));
            let mut validation_errors = validify::ValidationErrors::default();
            validation_errors.add(validation_error);
            validation_errors
        }
    }
}
