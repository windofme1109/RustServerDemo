use crate::dbaccess::course::{get_course_detail_db, post_new_course_db, update_course_detail_db, delete_course_db};
use crate::errors::MyError;
use crate::models::course::UpdateCourse;
use crate::{models::course::CreateCourse, state::AppState};
use actix_web::web;
use actix_web::HttpResponse;

pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    // 调用数数据库操作函数
    // let course = post_new_course_db(&app_state.db, new_course.into()).await;

    // HttpResponse::Ok().json(course)

    post_new_course_db(&app_state.db, new_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn get_course_detail(
    param: web::Path<(i32, i32)>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = param.into_inner();

    get_course_detail_db(&app_state.db, course_id, teacher_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn update_course_detail(
    update_course: web::Json<UpdateCourse>,
    param: web::Path<(i32, i32)>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    let (teacher_id, course_id) = param.into_inner();

    update_course_detail_db(
        &app_state.db,
        course_id,
        teacher_id,
        update_course.into_inner(),
    )
    .await
    .map(|course| HttpResponse::Ok().json(course))
}


pub async fn delete_course(
    param: web::Path<(i32, i32)>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {

    let (teacher_id, course_id) = param.into_inner();

    delete_course_db(&app_state.db, course_id, teacher_id).await.map(|res| {
        HttpResponse::Ok().json(res)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AppState;
    use actix_web::http::StatusCode;
    use actix_web::web;
    use actix_web::HttpResponse;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {
        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件中");

        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let visit_count = Mutex::new(0);
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            db: db_pool,
            visit_count: visit_count,
        });

        let new_course = web::Json(CreateCourse {
            teacher_id: 3,
            time: None,
            name: "Backend Class".to_string(),
            description: Some("First Backend Class".to_string()),
            format: None,
            structure: None,
            duration: None,
            price: Some(1000),
            language: None,
            level: None,
        });

        let res = post_new_course(new_course, app_state).await.unwrap();

        assert_eq!(res.status(), StatusCode::OK)
    }
}
