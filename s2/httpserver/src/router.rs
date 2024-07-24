use std::io::Write;

use http::httprequest::HttpRequest;
use http::httprequest::{Method, Resource};

use crate::handler::{Handler, StaticPageHandler, WebServiceHandler, PageNotFoundHandler, StaticImageHandler};

// 定义路由
pub struct Router {

}


impl Router {
    pub fn router(req: HttpRequest, stream: &mut impl Write) {
        let method = &req.method;
        let resource = &req.resource;
        match method {
            Method::Get => {
                // get 请求
                // 获取路径
                match resource {
                    Resource::Path(p) => {
                        // 切分路径
                        let path_list: Vec<&str> = p.split('/').collect();
                        println!("{:?}", path_list);
                        // path 部分一版情况下，以 / 开头，使用 / 切分，则第一个元素为空
                        match path_list[1] {
                            "api" => {
                                // 调用 handler 进行处理
                                let response = WebServiceHandler::handler(&req);
                                response.send_response(stream);
                            },
                            "image" => {
                                // todo 返回二进制内容
                                // 调用 handler 进行处理
                                let response = StaticImageHandler::handler(&req);

                                response.send_response(stream);
                            },
                            _ => {
                                // 
                                let response = StaticPageHandler::handler(&req);
                                response.send_response(stream);
                            }
                        }

                    },
                    _ => {

                    }
                }
            },
            Method::Post => {

            },
            _ => {

            }
        }
    }
}