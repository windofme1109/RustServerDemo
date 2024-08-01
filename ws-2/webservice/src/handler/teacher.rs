use actix_web::{get, HttpResponse};

// 使用属性的方式配置路由
#[get("/")]
pub async fn get_testing() -> HttpResponse {
    HttpResponse::Ok().body("testing config routing")
}