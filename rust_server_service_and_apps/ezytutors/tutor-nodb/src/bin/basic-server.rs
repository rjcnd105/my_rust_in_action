use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::io;


// router 설정
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    // path: "/health"
    // method: web::get()
    // handler: .to(health_check_handler)
    cfg.route("/health", web::get().to(health_check_handler));
}

// handler 설정
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. EzyTutors is alive and kicking")
}

// cargo run -p tutor-nodb --bin basic-server
// -p는 작업 공간 내에 바이너리를 빌드하고 실행하도록 하는 flag

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // move 키워드는 소유권(ownership)을 클로저로 이동시켜버려서 이 이후에 사용할 수 없게 함.
    let app = move || App::new().configure(general_routes);

    HttpServer::new(app).bind("127.0.0.1:3003")?.run().await
}
