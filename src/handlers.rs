mod auth;
mod root;
mod tasks;

pub use auth::login::login_form_handler;
pub use auth::login::login_handler;
pub use auth::logout::logout_handler;
pub use root::root_handler;
pub use tasks::delete_task::delete_task_handler;
pub use tasks::detail::task_detail_handler;
pub use tasks::edit_task::edit_task_form_handler;
pub use tasks::edit_task::update_task_handler;
pub use tasks::list::task_list_handler;
pub use tasks::new_task::create_task_handler;
pub use tasks::new_task::new_task_form_handler;
