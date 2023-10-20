mod app_error;
mod response_error;

pub use app_error::AppError;
pub type AppResult<T> = error_stack::Result<T, app_error::AppError>;
pub use response_error::AppResponseError;
