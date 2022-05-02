use actix_web::{HttpResponse, web};
use chrono::Utc;
use crate::models::Course;
use super::state::AppState;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse  {
    let health_check_response = &app_state.health_check_response;
    // 공유 가변 상태를 나타내는 필드(visit_count)는 액세스하기 전에 먼저 잠가야 여러 스레드가 필드 값을 동시에 업데이트하는 것을 방지할 수 있습니다.
    let mut visit_count = app_state.visit_count.lock().unwrap();
    // 클라이언트로 보낼 문자열
    let response = format!("{} {} times", health_check_response, visit_count);
    // 현재 데이터 잠금 중이므로 visit_count를 안전하게 변경할 수 있다.
    // 함수가 끝나면 데이터 잠금이 자동으로 해제된다.
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn get_courses_for_tutor(app_state: web::Data<AppState>, params: web::Path<(usize)>,) -> HttpResponse {
    let (tutor_id): usize = params.into_inner();

    println!("tutor_id: {}", tutor_id);

    let filtered_courses = app_state.courses
        .lock().unwrap().clone()
        .into_iter()
        .filter(|course| course.tutor_id == tutor_id)
        .collect::<Vec<Course>>(); // turbofish 라는 문법으로 타입을 지정해준다. typescript의 as 라고 보면 됌

    if filtered_courses.len() > 0 {
        HttpResponse::Ok().json(filtered_courses)
    } else {
        HttpResponse::Ok().json("No courses found for tutor")
    }
}

pub async fn new_courses(new_course: web::Json<Course>, app_state: web::Data<AppState>) -> HttpResponse {
    println!("Received new course");
    let course_count_for_user = app_state.courses.lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == new_course.tutor_id)
        .collect::<Vec<Course>>()
        .len();

    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some(course_count_for_user + 1),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc())
    };
    // 앱 상태에 courses에 course 추가
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Added course")
}

// #[cfg(test)]는 compile, run 때가 아닌 test명령시에만 실행하라는 의미
#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use actix_web::http::StatusCode;
    use actix_web::web;
    use crate::handlers::new_courses;
    use crate::models::Course;
    use crate::state::AppState;

    // async를 테스트하기 위한 attribute macro
    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(Course {
            tutor_id: 1,
            course_id: None,
            course_name: "Hello, this is test course".into(),
            posted_time: None
        });
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![])
        });
        let resp = new_courses(course, app_state);
        assert_eq!(resp.status(), StatusCode::OK)
    }

    // #[actix_rt::test]
    // async fn
}