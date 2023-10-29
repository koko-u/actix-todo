use actix_identity::Identity;
use actix_web::http;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web_flash_messages::FlashMessage;
use askama_actix::TemplateToResponse;

use crate::dtos::IntoTemplate;
use crate::dtos::IsValidStatus;
use crate::dtos::UpdateTask;
use crate::errors::AppResponseError;
use crate::models::UserModel;
use crate::states::AppState;
use crate::states::DbRepository;
use crate::templates::EditTaskTemplate;

pub async fn edit_task_form_handler<Repo>(
    app_data: web::Data<AppState<Repo>>,
    path: web::Path<i64>,
    identity: Option<Identity>,
) -> Result<impl Responder, AppResponseError>
where
    Repo: DbRepository,
{
    let id = path.into_inner();
    let user = UserModel::try_from_identity(identity, &app_data.repo).await?;
    let statuses = app_data.repo.get_all_statuses().await?;
    let Some(task) = app_data.repo.get_task_by_id(id).await? else {
        let redirect = HttpResponse::SeeOther()
            .append_header((http::header::LOCATION, "/tasks"))
            .body(format!("Task with id: {id} not found"));
        return Ok(redirect.map_into_left_body());
    };

    let show_form = EditTaskTemplate {
        statuses,
        login_user: user,
        id,
        summary: task.summary.into(),
        description: task.description.unwrap_or_default().into(),
        status: task.status_id.into(),
    };

    Ok(show_form.to_response().map_into_right_body())
}

pub async fn update_task_handler<Repo>(
    app_data: web::Data<AppState<Repo>>,
    form: web::Form<UpdateTask>,
    path: web::Path<i64>,
) -> Result<impl Responder, AppResponseError>
where
    Repo: DbRepository,
{
    use validify::Validify;
    let mut update_task = form.into_inner();
    let id = path.into_inner();

    let statuses = app_data.repo.get_all_statuses().await?;

    match (
        update_task.validify_self(),
        update_task.is_valid_status(&statuses),
    ) {
        (Ok(()), status_id_errors) if status_id_errors.is_empty() => {
            // validation all ok
            match app_data.repo.update_task(id, &update_task).await? {
                Some(updated_task) => {
                    FlashMessage::success(format!(
                        "Task #{} is successfully updated",
                        updated_task.id
                    ))
                    .send();
                }
                None => {
                    FlashMessage::error("Failed to update the task").send();
                }
            }
            let res = HttpResponse::SeeOther()
                .append_header((http::header::LOCATION, "/tasks"))
                .finish();
            Ok(res.map_into_right_body())
        }
        (Ok(()), status_id_errors) => {
            log::error!("Validation Error: {status_id_errors:?}");
            let error_part = update_task.into_template(status_id_errors);
            let res = EditTaskTemplate {
                statuses,
                id,
                ..error_part
            };
            Ok(res.to_response().map_into_left_body())
        }
        (Err(mut validation_errors), status_id_errors) => {
            // validation error
            validation_errors.merge(status_id_errors);
            log::error!("Validation Error: {validation_errors:?}");
            let error_part = update_task.into_template(validation_errors);
            let res = EditTaskTemplate {
                statuses,
                id,
                ..error_part
            };
            Ok(res.to_response().map_into_left_body())
        }
    }
}
