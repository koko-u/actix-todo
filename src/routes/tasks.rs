use actix_web::web;

use crate::handlers::create_task_handler;
use crate::handlers::edit_task_form_handler;
use crate::handlers::new_task_form_handler;
use crate::handlers::task_detail_handler;
use crate::handlers::task_list_handler;
use crate::handlers::update_task_handler;

pub fn tasks(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            .route("", web::get().to(task_list_handler))
            .route("", web::post().to(create_task_handler))
            .route("/new", web::get().to(new_task_form_handler))
            .route("/{id}", web::get().to(task_detail_handler))
            .route("/{id}", web::post().to(update_task_handler))
            .route("/{id}/edit", web::get().to(edit_task_form_handler)),
    );
}
