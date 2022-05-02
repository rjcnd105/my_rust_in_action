use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};



// tutor_id: 강의를 제공하는 튜터를 나타냅니다.
// course_id: 코스의 고유 식별자입니다. 우리 시스템에서 코스 ID는 튜터에게 고유합니다.
// course_name: 튜터가 제공하는 강좌명
// posted_time: 웹 서비스에서 코스를 기록한 타임스탬프입니다.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub tutor_id: usize,
    pub course_id: Option<usize>,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>

}

// 들어오는 HTTP 요청에서 Rust 구조체로 데이터를 변환하는 함수
impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            tutor_id: course.tutor_id,
            course_id: course.course_id,
            course_name: course.course_name.clone(),
            posted_time: course.posted_time
        }
    }
}