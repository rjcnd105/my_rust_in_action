// [컴파일러] 사용하지 않는 변수 있음 에러
#![allow(unused_variables)]
use std::fmt;
pub enum FileState {
    Open,
    Closed,
}

// 구조체 정의
// 수명도 포함시킬 수 있음
// print!와 같이 작동할 수 있게 함
// #[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

impl fmt::Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write! 매크로를 통해 디스플레이 구현을 따르는 것이 일반적이다.
        // *은 역참조다. deref
        // std::ops::Deref::deref trait을 통해 deref 액션을 구현할 수 있다.
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "OPEN"),
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state,)
    }
}

pub trait Read {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}
impl Read for File {
    fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        Ok(0)
    }
}

impl File {
    /// 이름만 지정할 시 새로 만드는 데이터는 비어있다.
    /// Creates a new, empty `File`.
    ///
    /// # Examples
    ///
    /// ```
    /// let f = File::new("f1.txt");
    /// ```
    pub fn new(name: &str) -> Self {
        Self::new_with_data(name, &Vec::new())
    }
    pub fn new_with_data(name: &str, data: &Vec<u8>) -> Self {
        Self {
            name: String::from(name),
            data: data.clone(),
            state: FileState::Closed,
        }
    }

    // 아무것도 반환하지 않음은 (), 여기서는 명시적으로 작성해줌.
    pub fn clear(text: &mut String) -> () {
        *text = String::from("");
    }
}
// fn read(self: &Self, save_to: &mut Vec<u8>) {
//     // Vector::clone : 복사본 반환
//     let mut temp = f.data.clone();
//     let read_length = temp.len();
//     println!("read, before temp: {:?}", temp);
//
//     // Vector::reserve : 공간을 예약한다. 빈번한 재할당을 막아 메모리를 효율적으로 쓰기 위함.
//     save_to.reserve(read_length);
//
//     // Vector::append : temp에 있는 값을 비우고 save_to로 이전한다.
//     save_to.append(&mut temp);
//
//     println!("read, save_to: {:?}", save_to);
//     println!("read, temp: {:?}", temp);
//     read_length
// }

pub fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

pub fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

// [컴파일러] 사용하지 않아도 허용
#[allow(dead_code)]
// -> ! 반환은 never, 반환하지 않음을 말함.

pub fn report<T: fmt::Debug>(item: T) {
    println!("{:?}", item);
}
