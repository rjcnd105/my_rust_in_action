// [컴파일러] 사용하지 않는 변수 있음 에러
#![allow(unused_variables)]

use rand;
use rand::{thread_rng, Rng};
use std::fmt::Debug;

enum FileState {
    Open,
    Closed,
}

// 구조체 정의
// 수명도 포함시킬 수 있음
// print!와 같이 작동할 수 있게 함
#[derive(Debug)]
pub struct File {
    pub name: String,
    pub data: Vec<u8>,
    pub state: FileState,
}

impl File {
    fn one_in(denominator: u32) -> bool {
        thread_rng().gen_ratio(1, denominator);
    }

    pub fn new(name: &str) -> File {
        File::new_with_data(name, &Vec::new())
    }
    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        File {
            name: String::from(name),
            data: data.clone(),
            state: FileState::Closed,
        }
    }
    pub fn read(self: &Self, save_to: &mut Vec<u8>) -> usize {
        // Vector::clone : 복사본 반환
        let mut temp = f.data.clone();
        let read_length = temp.len();
        println!("read, before temp: {:?}", temp);

        // Vector::reserve : 공간을 예약한다. 빈번한 재할당을 막아 메모리를 효율적으로 쓰기 위함.
        save_to.reserve(read_length);

        // Vector::append : temp에 있는 값을 비우고 save_to로 이전한다.
        save_to.append(&mut temp);

        println!("read, save_to: {:?}", save_to);
        println!("read, temp: {:?}", temp);
        read_length
    }

    // 아무것도 반환하지 않음은 (), 여기서는 명시적으로 작성해줌.
    pub fn clear(text: &mut String) -> () {
        *text = String::from("");
    }
}

pub fn open(mut f: File) -> Result<File, String> {
    if File::one_in(10_000) {
        let error_message = String::from("permission denied");
        return Err(error_message);
    }
    f.state = FileState::Open;
    Ok(f)
}

pub fn close(mut f: File) -> Result<File, String> {
    if File::one_in(10_000) {
        let error_message = String::from("permission denied");
        return Err(error_message);
    }
    Ok(f)
}

// [컴파일러] 사용하지 않아도 허용
#[allow(dead_code)]
// -> ! 반환은 never, 반환하지 않음을 말함.

pub fn report<T: Debug>(item: T) {
    println!("{:?}", item);
}
