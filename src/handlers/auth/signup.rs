use actix_identity::Identity;
use actix_web::http;
use actix_web::web;
use actix_web::HttpMessage;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use askama_actix::TemplateToResponse;
use error_stack::ResultExt;
use validify::Validate;

use crate::dtos::IntoTemplate;
use crate::dtos::NewUser;
use crate::errors::AppError;
use crate::errors::AppResponseError;
use crate::states::AppState;
use crate::states::DbRepository;
use crate::templates::SignUpTemplate;

pub async fn signup_form_handler<Repo>() -> Result<impl Responder, AppResponseError>
where
    Repo: DbRepository,
{
    Ok(SignUpTemplate::default())
}

pub async fn signup_handler<Repo>(
    request: HttpRequest,
    app_data: web::Data<AppState<Repo>>,
    payload: web::Form<NewUser>,
) -> Result<impl Responder, AppResponseError>
where
    Repo: DbRepository,
{
    let new_user = payload.into_inner();

    // form validation
    if let Err(validation_errors) = new_user.validate() {
        return Ok(new_user
            .into_template(validation_errors)
            .to_response()
            .map_into_left_body());
    }

    // signup user
    let new_user = app_data.repo.create_user(&new_user).await?;

    // login success
    Identity::login(&request.extensions(), format!("{}", new_user.id)).change_context(AppError)?;

    Ok(HttpResponse::SeeOther()
        .append_header((http::header::LOCATION, "/"))
        .finish()
        .map_into_right_body())
}
