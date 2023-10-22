use std::str;

use actix_web::cookie::Key;
use actix_web::middleware;
use actix_web::test::call_and_read_body;
use actix_web::test::init_service;
use actix_web::test::TestRequest;
use actix_web::web;
use actix_web::App;
use assertables::assert_contains;
use assertables::assert_contains_as_result;
use assertables::assert_not_contains;
use assertables::assert_not_contains_as_result;
use error_stack::ResultExt;

use todoapp::assets::assets_service;
use todoapp::errors::AppError;
use todoapp::errors::AppResult;
use todoapp::middlewares::build_flash;
use todoapp::routes::tasks;

use test_state::create_test_state;

#[path = "./mock/test_state.rs"]
mod test_state;

#[actix_web::test]
async fn test_get_tasks() -> AppResult<()> {
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

    let req = TestRequest::get().uri("/tasks").to_request();
    let resp = call_and_read_body(&app, req).await;
    let contents = str::from_utf8(&resp).change_context(AppError)?;

    assert_contains!(contents, "T1");
    assert_contains!(contents, "T2");
    assert_contains!(contents, "T3");
    assert_contains!(contents, "T4");
    assert_contains!(contents, "T5");
    assert_contains!(contents, "T6");
    assert_contains!(contents, "T7");
    Ok(())
}

#[actix_web::test]
async fn test_filter_tasks() -> AppResult<()> {
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

    let req = TestRequest::get().uri("/tasks?summary=T3").to_request();
    let resp = call_and_read_body(&app, req).await;
    let contents = str::from_utf8(&resp).change_context(AppError)?;

    assert_not_contains!(contents, "T1");
    assert_not_contains!(contents, "T2");
    assert_contains!(contents, "T3");
    assert_not_contains!(contents, "T4");
    assert_not_contains!(contents, "T5");
    assert_not_contains!(contents, "T6");
    assert_not_contains!(contents, "T7");
    Ok(())
}

#[actix_web::test]
async fn test_filter_status() -> AppResult<()> {
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

    let req = TestRequest::get().uri("/tasks?status_id=1&status_id=2").to_request();
    let resp = call_and_read_body(&app, req).await;
    let contents = str::from_utf8(&resp).change_context(AppError)?;

    assert_contains!(contents, "T1");
    assert_contains!(contents, "T2");
    assert_contains!(contents, "T3");
    assert_not_contains!(contents, "T4");
    assert_contains!(contents, "T5");
    assert_contains!(contents, "T6");
    assert_not_contains!(contents, "T7");
    Ok(())
}
