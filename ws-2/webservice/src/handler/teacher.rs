use actix_web::{delete, get, post, put, web, HttpResponse};
use crate::dbaccess::teacher::{get_all_teachers_db, post_new_teacher_db, get_teacher_detail_db, delete_teacher_db, update_teacher_detail_db};
use crate::models::teacher::{CreateTeacher, Teacher, UpdateTeacher};
use crate::{errors::MyError, state::AppState};

// 使用属性的方式配置路由
#[get("/")]
pub async fn get_all_teachers(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    get_all_teachers_db(&app_state.db).await.map(|teachers| {
        HttpResponse::Ok().json(teachers)
    })
}


#[post("/")]
pub async fn post_new_teacher(new_teacher: web::Json<CreateTeacher>, app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {

    post_new_teacher_db(&app_state.db, CreateTeacher::from(new_teacher)).await.map(|teacher| {
        HttpResponse::Ok().json(teacher)
    })
} 

#[get("/{teacher_id}")]
pub async fn get_teacher_detail(params: web::Path<i32>, app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    
    let teacher_id = params.into_inner();
    
    get_teacher_detail_db(&app_state.db, teacher_id).await.map(|teacher| {
        HttpResponse::Ok().json(teacher)
    })
}

#[delete("/{teacher_id}")]
pub async fn delete_teacher(params: web::Path<i32>, app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    delete_teacher_db(&app_state.db, teacher_id).await.map(|teacher| {
        HttpResponse::Ok().json(teacher)
    })
}


#[put("/{teacher_id}")]
pub async fn update_teacher_detail(update_teacher: web::Json<UpdateTeacher>, params: web::Path<i32>, app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {

    let teacher_id = params.into_inner();

    let teacher = UpdateTeacher::from(update_teacher);

    update_teacher_detail_db(&app_state.db, teacher_id, teacher).await.map(|teacher| {
        HttpResponse::Ok().json(teacher)
    })
} 