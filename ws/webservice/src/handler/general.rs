use std::sync::Mutex;

use crate::state::AppState;
use actix_web::{http::StatusCode, web::{self, Json}, App, HttpResponse};
use chrono::Utc;
use crate::errors::MyError;
use crate::models::course::Course;
use crate::dbaccess::course::*;



pub async fn health_check_handler(
    app_state: web::Data<AppState>
) -> HttpResponse {

    println!("access health");

    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();

    let response = format!("{} {} times", health_check_response, visit_count);

    *visit_count += 1;

    HttpResponse::Ok().json(&response)

}