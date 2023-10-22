use actix_web::http;
use actix_web::web;
use actix_web::Either;
use actix_web::HttpResponse;
use actix_web::Responder;

use crate::errors::AppResponseError;
use crate::states::AppState;
use crate::states::DbRepository;
use crate::templates::TaskDetail;

pub async fn task_detail_handler<Repo>(
    app_data: web::Data<AppState<Repo>>,
    path: web::Path<i64>,
) -> Result<impl Responder, AppResponseError>
where
    Repo: DbRepository,
{
    let id = path.into_inner();
    let task = app_data.repo.get_task_by_id(id).await?;
    let response = match task {
        Some(task) => Either::Right(TaskDetail { task: task.into() }),
        None => Either::Left(
            HttpResponse::SeeOther()
                .append_header((http::header::LOCATION, "/tasks"))
                .body(format!("Task with id: {id} not found")),
        ),
    };
    Ok(response)
}
