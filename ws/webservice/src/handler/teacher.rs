use crate::models::teacher::{Teacher, CreateTeacher, UpdateTeacher};
use crate::errors::MyError;
use crate::dbaccess::teacher::*;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn get_all_teachers(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    get_all_teachers_db(&app_state.db)
    .await
    .map(|teachers| HttpResponse::Ok().json(teachers))
} 


pub async fn get_teacher_details(app_state: web::Data<AppState>, params: web::Path<i32>) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    get_teacher_details_db(&app_state.db, teacher_id)
    .await
    .map(|teacher| HttpResponse::Ok().json(teacher))

}

pub async fn post_new_teacher(new_teacher:web::Json<CreateTeacher>, app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    
    post_new_teacher_db(&app_state.db, CreateTeacher::from(new_teacher))
    .await
    .map(|teacher| HttpResponse::Ok().json(teacher))

}

pub async fn update_teacher_details(app_state: web::Data<AppState>, params: web::Path<i32>, update_teacher: web::Json<UpdateTeacher>) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    update_teacher_details_db(
        &app_state.db,
        teacher_id,
        UpdateTeacher::from(update_teacher)
    )
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}


pub async fn delete_teacher(app_state: web::Data<AppState>, params: web::Path<i32>) -> Result<HttpResponse,MyError>{
    let teacher_id = params.into_inner();
    delete_teacher_db(&app_state.db, teacher_id)
    .await
    .map(|teacher| HttpResponse::Ok().json(teacher)) 
}
