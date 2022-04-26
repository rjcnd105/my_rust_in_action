use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;
pub struct Router;

/*
1. WebServiceHandler - to serve json data
2. StaticPageHandler - to serve static web pages,
3. PageNotFoundHandler - to serve 404 page
*/
impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            // Get Method 인 경우
            httprequest::Method::Get => match &req.resource {
                httprequest::Resource::Path(s) => {
                    // parse the uri
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        // Path가 "/api"로 시작하는 경우
                        // 1. WebServiceHandler 호출
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                        // 2. StaticPageHandler 호출
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            // Get Method가 아닌 경우
            // 3. PageNotFoundHandler 호출
            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}