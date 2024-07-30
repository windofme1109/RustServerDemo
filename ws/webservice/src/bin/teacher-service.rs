use actix_cors::Cors;
use actix_web::http;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use errors::MyError;
use std::env;
use std::io;

use sqlx::postgres::PgPoolOptions;
use std::sync::Mutex;

#[path = "../handler/mod.rs"]
mod handler;

#[path = "../routers.rs"]
mod routers;

#[path = "../state.rs"]
mod state;

#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../errors.rs"]
mod errors;
#[path = "../models/mod.rs"]
mod models;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件中 ");

    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    // 可以在 Actix Web App 中共享的数据
    let shared_data = web::Data::new(AppState {
        health_check_response: "I am OK".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![])
        db: db_pool,
    });

    let app = move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:9009/")
            .allowed_origin_fn(|origin, _reg_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                MyError::InvalidInput("Please provide valid Json input".to_string()).into()
            }))
            .wrap(cors)
            .configure(general_routes)
            .configure(course_routes)
            .configure(teacher_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:9009")?.run().await
    
}
