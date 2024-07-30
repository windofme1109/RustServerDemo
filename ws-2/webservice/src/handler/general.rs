use actix_web::web;
use actix_web::{HttpResponse, http::header::ContentType};
use crate::state::AppState;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    println!("access health checker");

    let health_check_response = &app_state.health_check_response;

    let mut visit_count = app_state.visit_count.lock().unwrap();

    let response = format!("{} {} times", health_check_response, visit_count);


    *visit_count += 1;

    HttpResponse::Ok().content_type(ContentType::json()).json(&response)
}