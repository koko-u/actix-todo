use actix_web::web;

use crate::errors::AppResult;
use crate::handlers::*;
use crate::states::AppState;
use crate::states::DbRepository;

pub fn auth<F, Repo>(_: F) -> impl FnOnce(&mut web::ServiceConfig)
where
    F: FnOnce() -> AppResult<AppState<Repo>>,
    Repo: DbRepository + 'static,
{
    move |cfg: &mut web::ServiceConfig| {
        cfg.service(
            web::scope("/auth")
                .route("/login", web::get().to(login_form_handler::<Repo>))
                .route("/login", web::post().to(login_handler::<Repo>))
                .route("/logout", web::post().to(logout_handler::<Repo>))
                .route("/signup", web::get().to(signup_form_handler::<Repo>))
                .route("/signup", web::post().to(signup_handler::<Repo>)),
        );
    }
}
