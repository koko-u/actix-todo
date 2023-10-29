use actix_web::web;

use crate::errors::AppResult;
use crate::handlers::root_handler;
use crate::states::AppState;
use crate::states::DbRepository;

pub fn root<F, Repo>(_: F) -> impl FnOnce(&mut web::ServiceConfig)
where
    F: FnOnce() -> AppResult<AppState<Repo>>,
    Repo: DbRepository + 'static,
{
    move |cfg: &mut web::ServiceConfig| {
        cfg.route("/", web::get().to(root_handler::<Repo>));
    }
}
