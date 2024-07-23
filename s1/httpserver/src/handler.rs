use http::httpresponse;
use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use core::str;
use std::collections::HashMap;
use std::env;
use std::fs;


pub trait Handler {
    fn handler(req: &HttpRequest) -> HttpResponse;

    fn load_file(file_name: &str) -> Option<String> {
        // 在编译的过程中，Cargo 会对外暴露一些环境变量，如 CARGO_MANIFEST_DIR 表示当前的这个 crate 的路径
        // 取这些变量需要使用 env 这个宏
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        // env::var 这个函数用于获取当前进程中的环境变量，如果没有，就取 default_path
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

        let full_path = format!("{}/{}", public_path, file_name);
        let contents = fs::read_to_string(full_path);

        return contents.ok()
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
        return HttpResponse::new("404", None, Self::load_file("404.html"))
    }
}

impl Handler for StaticPageHandler {
    fn handler(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;

        let route: Vec<&str> = s.split("/").collect();

        match route[1] {
            "" => {
                HttpResponse::new("200", None, Self::load_file("index.html"))
            }
            "health" => {
                HttpResponse::new("200", None, Self::load_file("health.html"))

            }
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }

                    HttpResponse::new("200", Some(map), Some(contents))

                }

                None => {
                    HttpResponse::new("404", None, Self::load_file("404.html"))

                }
            }
        }

    }
}


impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json");
        let json_content = fs::read_to_string(full_path);
        let orders: Vec<OrderStatus> = 
            serde_json::from_str(json_content.unwrap().as_str()).unwrap();

        return orders;
    }
 }

 impl Handler for WebServiceHandler {
    fn handler(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split("/").collect();
        // 匹配的是路径的第三部分：localhost:3000/api/shipping/orders
        match route[2] {
            "shipping" if route.len() > 2 && route[3] == "orders" => {
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            }
            _ => {
                HttpResponse::new("404", None, Self::load_file("404.html"))

            }
        }
    }
 }