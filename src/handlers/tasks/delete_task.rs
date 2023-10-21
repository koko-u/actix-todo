use actix_web::http;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web_flash_messages::FlashMessage;

use crate::errors::AppResponseError;
use crate::states::AppState;

pub async fn delete_task_handler(
    app_data: web::Data<AppState>,
    path: web::Path<i64>,
) -> Result<impl Responder, AppResponseError> {
    let id = path.into_inner();
    match app_data.db.delete_task(id).await? {
        Some(deleted_task) => {
            FlashMessage::success(format!("Task #{} is successfully deleted", deleted_task.id)).send();
        }
        None => {
            FlashMessage::error("Failed to delete the task").send();
        }
    }
    let res = HttpResponse::SeeOther()
        .append_header((http::header::LOCATION, "/tasks"))
        .finish();
    Ok(res)
}
