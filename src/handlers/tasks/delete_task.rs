use actix_identity::Identity;
use actix_web::http;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web_flash_messages::FlashMessage;

use crate::errors::AppResponseError;
use crate::models::UserModel;
use crate::states::AppState;
use crate::states::DbRepository;

pub async fn delete_task_handler<Repo>(
    app_data: web::Data<AppState<Repo>>,
    path: web::Path<i64>,
    identity: Option<Identity>,
) -> Result<impl Responder, AppResponseError>
where
    Repo: DbRepository,
{
    let id = path.into_inner();
    let user = UserModel::try_from_identity(identity, &app_data.repo).await?;
    if user.is_none() {
        // task list need to logged in.
        let res = HttpResponse::SeeOther()
            .append_header((http::header::LOCATION, "/auth/login"))
            .finish();
        return Ok(res.map_into_left_body());
    }

    match app_data.repo.delete_task(id).await? {
        Some(deleted_task) => {
            FlashMessage::success(format!("Task #{} is successfully deleted", deleted_task.id))
                .send();
        }
        None => {
            FlashMessage::error("Failed to delete the task").send();
        }
    }
    let res = HttpResponse::SeeOther()
        .append_header((http::header::LOCATION, "/tasks"))
        .finish();
    Ok(res.map_into_right_body())
}
