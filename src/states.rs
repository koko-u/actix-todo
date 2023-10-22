use error_stack::ResultExt;

use crate::errors::AppError;
use crate::errors::AppResult;

mod db;
mod repositories;

pub use repositories::StatusRepository;
pub use repositories::TasksRepository;

pub struct AppState {
    pub db: db::DbState,
}

impl AppState {
    pub fn new() -> AppResult<Self> {
        let database_url = dotenv::var("DATABASE_URL").change_context(AppError)?;
        let db = db::DbState::init(&database_url)?;

        Ok(Self { db })
    }
}
