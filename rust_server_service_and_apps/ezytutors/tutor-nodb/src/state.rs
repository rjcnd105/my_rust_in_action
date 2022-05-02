use std::sync::Mutex;
use crate::models::Course;

pub struct AppState {
    // 공유 immutable 상태
    pub health_check_response: String,
    // 공유 mutable 상태
    pub visit_count: Mutex<u32>,
    // 코스 컬렉션
    pub courses: Mutex<Vec<Course>>
}