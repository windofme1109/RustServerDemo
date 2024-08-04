use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use std::io;
use std::env;
use tera::Tera;

#[path = "../routers.rs"]
mod routers;

#[path = "../handlers.rs"]
mod handlers;

#[path = "../models.rs"]
mod models;

#[path = "../errors.rs"]
mod errors;

use routers::app_config;


#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let host_port = env::var("HOST_PORT").expect("HOST_PORT 没有在 .env 文件中");

    HttpServer::new(move || {
        // 生成 Tera 模板文件路径
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();

        let web_data = web::Data::new(tera);
        App::new()
            .app_data(web_data.clone())
            .configure(app_config)
    })
        .bind(host_port)?
        .run()
        .await
}

// use tera::Context;
//
// fn main() {
//
//
//
//     let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
//
//     let cxt = Context::new();
//
//     let template = tera.render("index.html", &cxt).unwrap();
//
//     println!("{}", template);
// }