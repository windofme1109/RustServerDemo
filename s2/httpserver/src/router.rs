use std::io::Write;

use http::httprequest::HttpRequest;
use http::httprequest::{Method, Resource};

// 定义路由
pub struct Router {

}


impl Router {
    pub fn router(req: HttpRequest, stream: &mut impl Write) {
        let method = req.method;
        let resource = req.resource;
        match method {
            Method::Get => {
                // get 请求
                // 获取路径
                match resource {
                    Resource::Path(p) => {
                        // 切分路径
                        let path_list: Vec<&str> = p.split('/').collect();
                        // path 部分一般是完整的 url，使用 / 切分，第一个元素是域名，从第二个元素开始，才是路径部分
                        match path_list[1] {
                            "api" => {
                                // 调用 handler 进行处理
                            },
                            "image" => {

                            },
                            _ => {
                                // 
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