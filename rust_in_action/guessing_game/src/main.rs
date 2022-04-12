use std::cmp::Ordering;
// rand crate 사용
// Rng는 정수 생성기가 구현한 메소드들을 정의한 trait이며 해당 메소드들을 이용하기 위해서는 반드시 스코프 내에 있어야 한다.
use rand::Rng;
// 사용자 입력을 받는 io 라이브러리는 std라는 표준라이브러리에 있다.
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess 1~10");

    let secret_number = rand::thread_rng().gen_range(1..11);

    loop {
        // 사용자의 입력을 저장할 공간.
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // expect는 Result가 Err시 프로그램 작동을 멈추고 표시할 메세지. Result 타입의 경우 예외처리를 안하면 waring이 뜬다! Err일시 표시됨

        // parse()는 문자열을 숫자형으로 파싱. :u32 는 32비트의 정수인데, 이렇게 명시해준 숫자 타입으로 파싱함
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You Guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // loop를 빠져나온다.
            }
        }
    }
}

// Result는 Ok, Err로 구성된 enum이자 either
/*
pub enum Result<T, E> {
    /// Contains the success value
    #[lang = "Ok"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Ok(#[stable(feature = "rust1", since = "1.0.0")] T),

    /// Contains the error value
    #[lang = "Err"]
    #[stable(feature = "rust1", since = "1.0.0")]
    Err(#[stable(feature = "rust1", since = "1.0.0")] E),
}
 */
