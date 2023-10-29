pub trait IntoTemplate {
    type Template;
    fn into_template(self, validation_errors: validify::ValidationErrors) -> Self::Template;
}
