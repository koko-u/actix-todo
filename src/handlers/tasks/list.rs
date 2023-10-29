use actix_identity::Identity;
use actix_web::http;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web_flash_messages::IncomingFlashMessages;
use askama_actix::TemplateToResponse;

use crate::dtos::TaskFilter;
use crate::errors::AppResponseError;
use crate::models::UserModel;
use crate::states::AppState;
use crate::states::DbRepository;
use crate::templates::TaskList;

// GET /tasks
pub async fn task_list_handler<Repo>(
    app_data: web::Data<AppState<Repo>>,
    query: web::Query<TaskFilter>,
    flash_messages: IncomingFlashMessages,
    identity: Option<Identity>,
) -> Result<impl Responder, AppResponseError>
where
    Repo: DbRepository,
{
    let user = UserModel::try_from_identity(identity, &app_data.repo).await?;
    if user.is_none() {
        // task list need to logged in.
        let res = HttpResponse::SeeOther()
            .append_header((http::header::LOCATION, "/auth/login"))
            .finish();
        return Ok(res.map_into_left_body());
    }

    let user_id = user.as_ref().map(|user| user.id).unwrap_or_default();
    let task_filter = query.into_inner();

    let tasks = app_data.repo.get_filtered_tasks(user_id, &task_filter).await?;
    let statuses = app_data.repo.get_all_statuses().await?;

    use actix_web_flash_messages::Level as FLevel;
    let task_list = TaskList {
        task_filter,
        login_user: user,
        tasks: tasks.into_iter().map(From::from).collect(),
        statuses,
        success_flash_messages: flash_messages.filter(FLevel::Success),
        error_flash_messages: flash_messages.filter(FLevel::Error),
    };

    Ok(task_list.to_response().map_into_right_body())
}

trait FilterMessage {
    fn filter(&self, level: actix_web_flash_messages::Level) -> Vec<String>;
}
impl FilterMessage for IncomingFlashMessages {
    fn filter(&self, level: actix_web_flash_messages::Level) -> Vec<String> {
        self.iter()
            .filter(|message| message.level() == level)
            .map(|message| message.content().into())
            .collect()
    }
}
