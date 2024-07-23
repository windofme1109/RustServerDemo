// 对每个路由进行处理

use std::env;
use std::fs;
use http::httprequest::Resource;
use http::{httprequest::HttpRequest, httpresponse::HttpResponse};

use serde::{Serialize, Deserialize};

pub trait Handler {
    fn handler(req: &HttpRequest) -> HttpResponse;

    fn load_file(filename: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));

        // 从环境变量中尝试读取公共资源路径，如果没有，则取默认值
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

        let file_path = format!("{}/{}", public_path, filename);

        // 读取文件内容，并将其转换为字符串
        let content = fs::read_to_string(file_path);

        // 返回文件内容
        content.ok()
    }
}


pub struct StaticPageHandler; 
pub struct PageNotFoundHandler; 
pub struct WebServiceHandler; 

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String
}

impl Handler for PageNotFoundHandler {
    fn handler(req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

impl Handler for StaticPageHandler {
    fn handler(req: &HttpRequest) -> HttpResponse {
        let Resource::Path(s) = &req.resource;

        let path_list: Vec<&str> = s.split("/").collect();

        match path_list[1] {
            "" => {
                HttpResponse::new("200", None, Self::load_file("inde.html"))

            },
            "health" => {
                HttpResponse::new("200", None, Self::load_file("health.html"))

            },
            path => {
                // 其他路径进行处理
            }
        }
    }
}