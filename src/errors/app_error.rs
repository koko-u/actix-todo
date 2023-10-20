#[derive(Debug, derive_more::Display)]
#[display("Application Error")]
pub struct AppError;

impl error_stack::Context for AppError {}
