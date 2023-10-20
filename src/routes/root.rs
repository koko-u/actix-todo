use actix_web::web;

use crate::handlers::root_handler;

pub fn root(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(root_handler));
}
