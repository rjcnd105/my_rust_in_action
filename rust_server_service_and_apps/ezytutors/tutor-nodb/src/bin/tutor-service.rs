

#[path = "../models.rs"]
mod models;
#[path = "../handlers.rs"]
mod handlers;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;

use std::io;
use std::sync::Mutex;
use actix_web::{App, HttpServer, web};
use routes::*;
use state::AppState;

// 비동기 실행
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 어플리케이션 상태 초기화
    let shared_data = web::Data::new(AppState {
        health_check_response: "i'm good. You've already asked me".to_string(),
        visit_count: Mutex::new(0),
        courses: Mutex::new(vec![])
    });
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes) // 경로 구성
            .configure(course_routes) // 경로 구성
    };
    // keyword await
    // Future의 결과가 준비될때까지 실행을 일시 중단함.
    HttpServer::new(app).bind("127.0.0.1:3031")?.run().await
}