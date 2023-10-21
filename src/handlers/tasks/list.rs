use actix_web::web;
use actix_web::Responder;
use actix_web_flash_messages::IncomingFlashMessages;

use crate::dtos::TaskFilter;
use crate::errors::AppResponseError;
use crate::states::AppState;
use crate::templates::TaskList;
use crate::utils::AsOption;

// GET /tasks
pub async fn task_list_handler(
    app_data: web::Data<AppState>,
    query: web::Query<TaskFilter>,
    flash_messages: IncomingFlashMessages,
) -> Result<impl Responder, AppResponseError> {
    let TaskFilter { summary } = query.into_inner();
    let task_filter = TaskFilter {
        summary: summary.as_ref().and_then(AsOption::empty_as_none),
    };
    log::info!("{task_filter:#?}");
    let tasks = app_data.db.get_filtered_tasks(&task_filter).await?;

    use actix_web_flash_messages::Level as FLevel;
    let task_list = TaskList {
        search_key: task_filter.summary,
        tasks: tasks.into_iter().map(From::from).collect(),
        success_flash_messages: flash_messages.filter(FLevel::Success),
        error_flash_messages: flash_messages.filter(FLevel::Error),
    };

    Ok(task_list)
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
