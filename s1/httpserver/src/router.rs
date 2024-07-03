use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn router(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                                let resp = WebServiceHandler::handler(&req);
                                let _ = resp.send_response(stream);
                        }
                        _ => {
                                let resp = StaticPageHandler::handler(&req);
                                let _ = resp.send_response(stream);
                        }
                    }
                }
            } 
                  
            _ => {
                let resp = PageNotFoundHandler::handler(&req);
                let _ = resp.send_response(stream);
            }
        }

    
    }
}