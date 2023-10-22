use actix_web::web;

use crate::errors::AppResult;
use crate::handlers::*;
use crate::states::AppState;
use crate::states::DbRepository;

pub fn tasks<F, Repo>(_: F) -> impl FnOnce(&mut web::ServiceConfig)
where
    F: FnOnce() -> AppResult<AppState<Repo>>,
    Repo: DbRepository + 'static,
{
    move |cfg: &mut web::ServiceConfig| {
        cfg.service(
            web::scope("/tasks")
                .route("", web::get().to(task_list_handler::<Repo>))
                .route("", web::post().to(create_task_handler::<Repo>))
                .route("/new", web::get().to(new_task_form_handler::<Repo>))
                .route("/{id}", web::get().to(task_detail_handler::<Repo>))
                .route("/{id}", web::post().to(update_task_handler::<Repo>))
                .route("/{id}/edit", web::get().to(edit_task_form_handler::<Repo>))
                .route("/{id}/delete", web::post().to(delete_task_handler::<Repo>)),
        );
    }
}

