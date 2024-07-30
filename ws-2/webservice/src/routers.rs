use actix_web::web;

use crate::handler::general::*;

pub fn general_route(cfg: &mut web::ServiceConfig) {

    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_route(cfg: &mut web::ServiceConfig) {

    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn teacher_route(cfg: &mut web::ServiceConfig) {

    cfg.route("/health", web::get().to(health_check_handler));
}