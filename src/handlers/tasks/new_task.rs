use actix_identity::Identity;
use actix_web::http;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web_flash_messages::FlashMessage;
use askama_actix::TemplateToResponse;

use crate::dtos::IntoTemplate;
use crate::dtos::IsValidStatus;
use crate::dtos::NewTask;
use crate::errors::AppResponseError;
use crate::models::UserModel;
use crate::states::AppState;
use crate::states::DbRepository;
use crate::templates::NewTaskTemplate;

pub async fn new_task_form_handler<Repo>(
    app_data: web::Data<AppState<Repo>>,
    identity: Option<Identity>,
) -> Result<impl Responder, AppResponseError>
where
    Repo: DbRepository,
{
    let statuses = app_data.repo.get_all_statuses().await?;
    let user = UserModel::try_from_identity(identity, &app_data.repo).await?;
    Ok(NewTaskTemplate {
        statuses,
        login_user: user,
        ..Default::default()
    })
}

pub async fn create_task_handler<Repo>(
    app_data: web::Data<AppState<Repo>>,
    form: web::Form<NewTask>,
) -> Result<impl Responder, AppResponseError>
where
    Repo: DbRepository,
{
    use validify::Validify;
    let mut new_task = form.into_inner();
    let statuses = app_data.repo.get_all_statuses().await?;
    match (
        new_task.validify_self(),
        new_task.is_valid_status(&statuses),
    ) {
        (Ok(()), status_id_errors) if status_id_errors.is_empty() => {
            let new_task = app_data.repo.create_task(&new_task).await?;
            FlashMessage::success(format!("New task #{} has been created.", new_task.id)).send();
            let res = HttpResponse::SeeOther()
                .append_header((http::header::LOCATION, "/tasks"))
                .finish();
            Ok(res.map_into_right_body())
        }
        (Ok(()), status_id_errors) => {
            log::error!("Validation Error: {status_id_errors:#?}");
            let error_part = new_task.into_template(status_id_errors);
            let res = NewTaskTemplate {
                statuses,
                ..error_part
            };
            Ok(res.to_response().map_into_left_body())
        }
        (Err(mut validation_errors), status_id_errors) => {
            validation_errors.merge(status_id_errors);
            log::error!("Validation Error: {validation_errors:#?}");
            let error_part = new_task.into_template(validation_errors);
            let res = NewTaskTemplate {
                statuses,
                ..error_part
            };
            Ok(res.to_response().map_into_left_body())
        }
    }
}
