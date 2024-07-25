// 启动基本的服务
use actix_web::{App, HttpServer, HttpResponse, web};
use std::io::Result;


#[actix_rt::main]
async fn main() -> Result<()>{

    HttpServer::new(|| {
        App::new().route("/", web::get().to(HttpResponse::Ok))
    })
    .bind("localhost:10000")?
    .run()
    .await
    
}