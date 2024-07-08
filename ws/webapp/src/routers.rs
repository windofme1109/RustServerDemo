use crate::handlers::{};
use actix_web::{web};
use actix_files as fs;

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
        .service(fs::Files::new("/static", service(web::resource("/")
        .route(web::service(web::resource("/register") roservice(web::resource("/register-post

}

