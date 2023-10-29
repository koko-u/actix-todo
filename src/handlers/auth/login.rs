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
use validify::ValidationError;
use validify::ValidationErrors;

use crate::dtos::IntoTemplate;
use crate::dtos::LoginForm;
use crate::errors::AppError;
use crate::errors::AppResponseError;
use crate::states::AppState;
use crate::states::DbRepository;
use crate::templates::LoginTemplate;

pub async fn login_form_handler<Repo>(
    _app_data: web::Data<AppState<Repo>>,
) -> Result<impl Responder, AppResponseError>
where
    Repo: DbRepository,
{
    Ok(LoginTemplate::default())
}

pub async fn login_handler<Repo>(
    request: HttpRequest,
    app_data: web::Data<AppState<Repo>>,
    payload: web::Form<LoginForm>,
) -> Result<impl Responder, AppResponseError>
where
    Repo: DbRepository,
{
    let login_form = payload.into_inner();

    // form validation
    if let Err(validation_errors) = login_form.validate() {
        return Ok(login_form
            .into_template(validation_errors)
            .to_response()
            .map_into_left_body());
    }

    // password validation
    let Some(user) = app_data
        .repo
        .auth_user(&login_form.email, &login_form.password)
        .await?
    else {
        let mut login_error = ValidationError::new_field("password", "password");
        login_error.set_message("Invalid email or password".into());
        let mut login_errors = ValidationErrors::new();
        login_errors.add(login_error);

        return Ok(login_form
            .into_template(login_errors)
            .to_response()
            .map_into_left_body());
    };

    // login success
    Identity::login(&request.extensions(), format!("{}", user.id)).change_context(AppError)?;

    Ok(HttpResponse::SeeOther()
        .append_header((http::header::LOCATION, "/tasks"))
        .finish()
        .map_into_right_body())
}
