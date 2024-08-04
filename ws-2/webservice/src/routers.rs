use actix_web::{middleware, web};
use actix_web::http::Method;
use actix_web::middleware::TrailingSlash;
use crate::handler::general::*;
use crate::handler::course::*;
use crate::handler::teacher::*;


pub fn general_routes(cfg: &mut web::ServiceConfig) {

    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {

    cfg.service(
        web::scope("/course")
        .route("/", web::post().to(post_new_course))
        .route("/{teacher_id}/{course_id}", web::get().to(get_course_detail))
        .route("/{teacher_id}/{course_id}", web::put().to(update_course_detail))
        .route("/{teacher_id}/{course_id}", web::delete().to(delete_course))
    );
    
    
}

pub fn teacher_routes(cfg: &mut web::ServiceConfig) { cfg.service(

        // todo 使用 NormalizePath 中间件对尾斜杠进行处理
        web::scope("/teacher")
            .wrap(middleware::NormalizePath::new(TrailingSlash::Always))
            .service(get_all_teachers)
            .service(post_new_teacher)
            .service(get_teacher_detail)
            .service(update_teacher_detail)
            .service(delete_teacher)
            .default_service(web::route().method(Method::GET))
    );
    
}