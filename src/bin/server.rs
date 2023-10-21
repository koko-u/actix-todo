use std::env;
use std::net;

use actix_web::cookie::Key;
use actix_web::middleware;
use actix_web::web;
use actix_web::App;
use actix_web::HttpServer;
use env_logger::Env;
use error_stack::ResultExt;

use todoapp::assets::assets_service;
use todoapp::errors::AppError;
use todoapp::errors::AppResult;
use todoapp::middlewares;
use todoapp::routes::not_found_service;
use todoapp::routes::root;
use todoapp::routes::tasks;
use todoapp::states::AppState;

fn is_production() -> bool {
    env::var("APP_ENV").map_or(false, |app_env| app_env.to_lowercase() == "production")
}

#[actix_web::main]
async fn main() -> AppResult<()> {
    if !is_production() {
        // load .env only development enviornment
        dotenv::dotenv().change_context(AppError)?;
    }

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let addrs = net::SocketAddr::from(([127, 0, 0, 1], 8080));
    let app_data = web::Data::new(AppState::new()?);

    let cookie_key = dotenv::var("COOKIE_KEY").change_context(AppError)?;
    let cookie_key = Key::from(cookie_key.as_bytes());

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(middlewares::build_flash(cookie_key.clone()))
            .service(assets_service())
            .configure(root)
            .configure(tasks)
            .default_service(not_found_service())
    })
    .bind(addrs)
    .change_context(AppError)?
    .run()
    .await
    .change_context(AppError)?;

    Ok(())
}
