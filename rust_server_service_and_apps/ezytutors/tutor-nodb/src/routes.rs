use super::handlers;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(handlers::health_check_handler));
}

pub fn course_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // /courses와 관련된 모든 api를 추가할 수 있는 새로운 리소스 범위를 생성함
        web::scope("/courses")
            .route("/", web::post().to(new_course))
            .route("/{user_id}", web::get().to(get_courses_for_tutor))
            .route("/{user_id}/{course_id}", web::get().to(get_course_detail)),
    );
}
