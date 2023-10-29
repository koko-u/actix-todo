use actix_identity::Identity;
use actix_web::http;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use askama_actix::TemplateToResponse;

use crate::errors::AppResponseError;
use crate::models::UserModel;
use crate::states::AppState;
use crate::states::DbRepository;
use crate::templates::TaskDetail;

pub async fn task_detail_handler<Repo>(
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

    let task = app_data.repo.get_task_by_id(id).await?;
    match task {
        Some(task) => {
            let task_detail = TaskDetail {
                login_user: user,
                task: task.into(),
            };
            Ok(task_detail.to_response().map_into_right_body())
        }
        None => {
            let res = HttpResponse::SeeOther()
                .append_header((http::header::LOCATION, "/tasks"))
                .body(format!("Task with id: {id} not found"));
            Ok(res.map_into_left_body())
        }
    }
}
