use actix_web::web;
use actix_web::Route;

use crate::templates::NotFoundTemplate;

pub fn not_found_service() -> Route {
    web::to(|| async { NotFoundTemplate })
}
