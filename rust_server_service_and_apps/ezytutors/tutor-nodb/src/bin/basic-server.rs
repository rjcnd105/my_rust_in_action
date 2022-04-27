use actix_web::{HttpResponse, Responder, web};
use std::io;
use actix_web::web::ServiceConfig;

// router 설정
pub fn general_router(cfg: &mut web::ServiceConfig) -> &mut ServiceConfig {
    cfg.route("/health", web::get().to(health_check_handler))
}

// handler 설정
pub fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. EzyTutors is alive and kicking")
}

fn main() {

}