use std::env;

// 启动基本的服务
use actix_web::{App, HttpServer, HttpResponse, web};
use std::io;
use std::sync::Mutex;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;

#[path="../state.rs"]
mod state;
#[path= "../routers.rs"]
mod routers;

#[path="../handler/mod.rs"]
mod handler;

#[path ="../models/mod.rs"]
mod models;

#[path ="../dbaccess/mod.rs"]
mod dbaccess;

#[path ="../errors.rs"]
mod errors;

use state::AppState;
use routers::general_routes;
use routers::course_routes;
use routers::teacher_routes;

#[actix_rt::main]
async fn main() -> io::Result<()> {

    // 从 .env 文件中读取变量，读取成功以后，将这些变量注入操作系统的环境变量中
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件中");

    // 连接数据库
    let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();


    // 定义一个应用状态（Application State），状态数据可以被所有的路由以及同一范围（scope）内的资源共享
    // 使用 web::Data::new 创建一个状态
    // 使用 web::Data executor 访问数据
    let shared_data = web::Data::new(AppState {
        health_check_response: "I am OK".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool
    });

    let app = move || {

        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
            .configure(teacher_routes)
    };

    HttpServer::new(app)
    .bind("localhost:10000")?
    .run()
    .await

}