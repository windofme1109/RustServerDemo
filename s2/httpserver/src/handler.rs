// 对每个路由进行处理

use std::collections::HashMap;
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
pub struct StaticImageHandler; 

#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String
}

impl Handler for  StaticImageHandler {
    fn handler(req: &HttpRequest) -> HttpResponse {
        let Resource::Path(s) = &req.resource;
        let path_list = s.split("/").collect::<Vec<&str>>();

        //  请求 image 的示例：/image/cover.jpg
        match path_list[1] {
            "image" => {
                if path_list.len() > 2 {
                    let file_name = path_list[2];
                    let image_content = Self::load_image(file_name);
                    
                    let mut headers = HashMap::new();
                    headers.insert("content-type", "image/jpeg");

                    let response = HttpResponse::new("200", Some(headers), Some(image_content));

                    response
                } else {
                    let mut headers = HashMap::new();
                    headers.insert("content-type", "text/plain");

                    let response = HttpResponse::new("200", Some(headers), Some("Not Found Image".as_bytes().to_vec()));
                    
                    response
                }
            },
            _ => {
                let mut headers = HashMap::new();
                    headers.insert("content-type", "text/plain");

                    let response = HttpResponse::new("200", Some(headers), Some("Not Found Image".as_bytes().to_vec()));
                    
                    response
            }
        }
    }
}

impl StaticImageHandler {
    // 加载图片
    pub fn load_image(filename: &str) -> Vec<u8> {
        let default_path = format!("{}/assets", env!("CARGO_MANIFEST_DIR"));

        // 从环境变量中尝试读取公共资源路径，如果没有，则取默认值
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);

        let file_path = format!("{}/{}", public_path, filename);

        let data = fs::read(file_path).unwrap();

        data
    }
}

impl Handler for PageNotFoundHandler {
    fn handler(req: &HttpRequest) -> HttpResponse {
        HttpResponse::new("404", None, Some(Self::load_file("404.html").unwrap().into_bytes()))
    }
}

impl Handler for StaticPageHandler {
    fn handler(req: &HttpRequest) -> HttpResponse {
        let Resource::Path(s) = &req.resource;

        let path_list: Vec<&str> = s.split("/").collect();

        match path_list[1] {
            "" => {
                // Self 表示实现了 Handler trait 的结构体
                HttpResponse::new("200", None, Some(Self::load_file("index.html").unwrap().into_bytes()))

            },
            "health" => {
                HttpResponse::new("200", None, Some(Self::load_file("health.html").unwrap().into_bytes()))

            },
            // 其他路径进行处理
            path => {
                let content = Self::load_file(path);

                match content {
                    Some(c) => {
                        let mut map = HashMap::new();

                        if path.ends_with(".css") {
                            map.insert("Content-type", "text/css");
                        } else if path.ends_with(".js") {
                            map.insert("Content-type", "text/javascript");
                        } else {
                            map.insert("Content-type", "text/html");
                        }

                        HttpResponse::new("200", Some(map), Some(c.into_bytes()))
                    },
                    None => {
                        HttpResponse::new("404", None, Some(Self::load_file("404.html").unwrap().into_bytes()))

                    }
                }
                
            }
        }
    }
}


impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));

        // 从环境变量中尝试读取公共资源路径，如果没有，则取默认值
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);

        let json_file_path = format!("{}/{}", data_path, "orders.json");

        let content = fs::read_to_string(json_file_path).unwrap();

        // 文本转换为 json
        let orders: Vec<OrderStatus> = serde_json::from_str(content.as_str()).unwrap();
        

        orders
    }
}


impl Handler for WebServiceHandler {
    fn handler(req: &HttpRequest) -> HttpResponse {
        let Resource::Path(p) = &req.resource;

        let route: Vec<&str> = p.split("/").collect();

        if route.len() < 3 {
            // 单纯以 api 开头，没有任何后缀
            let mut headers = HashMap::new();
            headers.insert("Content-Type", "text/plain");
                    
            return HttpResponse::new("200", Some(headers), Some("No api found".as_bytes().to_vec()))
        } 

        // 匹配的是路径的第三部分：localhost:3000/api/shipping/orders
        match route[2] {
            "shipping" => {
                if route.len() > 3 && route[3] == "orders" {
                    let body = Some(serde_json::to_string(&Self::load_json()).unwrap().into_bytes());
                    
                    let mut headers = HashMap::new();
                    headers.insert("Content-Type", "application/json");
                    
                    HttpResponse::new("200", Some(headers), body)
                } else {
                    let mut headers = HashMap::new();
                    headers.insert("Content-Type", "text/plain");
                    HttpResponse::new("200", Some(headers), Some("Hello world".as_bytes().to_vec()))
                }
            },
            _ => {
                let mut headers = HashMap::new();
                    headers.insert("Content-Type", "text/plain");
                    HttpResponse::new("200", Some(headers), Some("No api found".as_bytes().to_vec()))
            }
        }

    }
}