fn main() {
    println!("Hello, world!");
}

#[test]
mod test {
    use std::num::ParseIntError;

    fn try_to_parse() -> Result<i32, ParseIntError> {
        let x: i32 = "123".parse()?; // x = 123
        let y: i32 = "24a".parse()?; // 여기서 바로 Err(ParseIntError { kind: InvalidDigit })가 리턴됌
        Ok(x + y)                    // Doesn't run.
    }

    fn try_to_parse2() -> (Result<u8, ParseIntError>, Result<u8, ParseIntError>) {
        let x: Result<u8, ParseIntError>= "123".parse(); // x = 123
        let y: Result<u8, ParseIntError>= "24a".parse(); // returns an Err() immediately
        (x, y)                    // Doesn't run.
    }

    // question operator: ?
    // Result나 Option에 사용하여 값을 바로 벗겨냄
    #[test]
    fn question_operator_test()  {
        let res = try_to_parse();
        println!("{:?}", res); // Err(ParseIntError { kind: InvalidDigit })

        let res = try_to_parse2();
        println!("{:?}", res); // (Ok(123), Err(ParseIntError { kind: InvalidDigit }))
    }
}