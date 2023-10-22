use actix_web::cookie::Key;
use actix_web::http;
use actix_web::http::StatusCode;
use actix_web::middleware;
use actix_web::test::call_service;
use actix_web::test::init_service;
use actix_web::test::TestRequest;
use actix_web::web;
use actix_web::App;
use error_stack::ResultExt;

use error_stack::bail;
use todoapp::assets::assets_service;
use todoapp::dtos::NewTask;
use todoapp::errors::AppError;
use todoapp::errors::AppResult;
use todoapp::middlewares::build_flash;
use todoapp::routes::tasks;

use test_state::create_test_state;

#[path = "./mock/test_state.rs"]
mod test_state;

#[actix_web::test]
async fn test_create_tasks() -> AppResult<()> {
    env_logger::init();

    let key = Key::generate();
    let app_state = create_test_state()?;
    let app_data = web::Data::new(app_state);

    let app = init_service(
        App::new()
            .app_data(app_data.clone())
            .wrap(build_flash(key.clone()))
            .wrap(middleware::Logger::default())
            .service(assets_service())
            .configure(tasks(create_test_state)),
    )
    .await;

    let create_task = NewTask {
        summary: "New Task".into(),
        description: "New Description".into(),
        status_id: 2,
    };

    let req = TestRequest::post()
        .uri("/tasks")
        .set_form(create_task)
        .to_request();
    let resp = call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::SEE_OTHER);
    let Some(loc) = resp.headers().get(http::header::LOCATION) else {
        bail!(AppError);
    };
    let loc = loc.to_str().change_context(AppError)?;
    assert_eq!(loc, "/tasks");

    Ok(())
}
