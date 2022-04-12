// Enums
// 패턴 매칭이랑 결합하여 강력하게 사용 가능
// struct와 마찬가지로 impl로 메소드 구현 가능
// constants 집합보다 강력함

#[derive(Debug)]
pub enum Event {
    Update,
    Delete,
    Unknown,
}

type Message = String;

pub fn parse_log(line: &str) -> (Event, Message) {
    // 최대 [&str, 2] 를 반환하는 벡터
    // Vec<_>에서 _는 컴파일러보고 타입을 추론하라는 의미
    let parts: Vec<_> = line.splitn(2, ' ').collect();

    if parts.len() == 1 {
        return (Event::Unknown, String::from(line));
    }

    let event = parts[0];
    let rest = String::from(parts[1]);

    match event {
        "UPDATE" | "update" => (Event::Update, rest),
        "DELETE" | "delete" => (Event::Delete, rest),
        _ => (Event::Unknown, rest),
    }
}

enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

// 변형 데이터를 포함하여 구조체와 같은 페르소나를 부여 가능... 이게 머시당까
enum Card {
    King(Suit),
    Queen(Suit),
    Jack(Suit),
    Ace(Suit),
    Pip(Suit, usize),
}
