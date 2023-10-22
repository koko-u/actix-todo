use error_stack::ResultExt;

use crate::errors::AppError;
use crate::errors::AppResult;

mod db;
mod repositories;

pub use repositories::DbRepository;

pub struct AppState<Repo> {
    pub repo: Repo,
}

pub fn create_app_state() -> AppResult<AppState<impl DbRepository>> {
    let database_url = dotenv::var("DATABASE_URL").change_context(AppError)?;
    let db = db::DbState::init(&database_url)?;
    Ok(AppState { repo: db })
}
