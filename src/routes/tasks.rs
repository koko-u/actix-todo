use actix_web::web;

use crate::handlers::*;

pub fn tasks(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .route("", web::get().to(task_list_handler))
            .route("", web::post().to(create_task_handler))
            .route("/new", web::get().to(new_task_form_handler))
            .route("/filter", web::get().to(filter_task_list_handler))
            .route("/{id}", web::get().to(task_detail_handler))
            .route("/{id}", web::post().to(update_task_handler))
            .route("/{id}/edit", web::get().to(edit_task_form_handler))
            .route("/{id}/delete", web::post().to(delete_task_handler)),
    );
}
