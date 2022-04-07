use std::time::Duration;

// for 문
pub fn for_the_central_pillar_of_iteration() {
    let container = [1, 2, 3, 4];
    let mut mutable_container = [11, 22, 33, 44];
    for item in container {}

    // for문에서 collection을 반복하는데는 3가지 방법이 있다.

    // Ownership only.
    // 이 경우 container은 해당 로컬 범위를 벗어나면 라이프 사이클이 끝난다.
    for item in container {
        // Equivalent to) for item in IntoIterator::into_iter(collection)
        println!("container {}", item);
    }

    // read-only
    // 이 경우 container의 라이프 사이클이 끝나지 않음
    for item in &container {
        // Equivalent to) for item in collection.iter()
        println!("&container {}", item);
    }

    // read-write
    // 단 mutable collection 인 경우만 가능
    for item in &mut mutable_container {
        // Equivalent to) for item in collection.iter_mut()
        println!("&mut mutable_container {}", item);
    }

    // 익명인 경우에 _를 사용한다.
    // 일정 횟수 만큼의 반복임을 강조함
    for _ in 0..10 {
        // ...
    }

    // 인덱스를 사용할 수도 있으나 권장되지 않는다.
    // 1. 퍼포먼스가 구림
    // 2. 안전하지 않음
    let collection = [1, 2, 3, 4, 5];
    for i in 0..collection.len() {
        let item = collection[i];
        // ...
    }

    // continue: 현재 반복을 스킵하고 다음 반복 진행
    for n in 0..5 {
        if n % 2 == 0 {
            continue;
        }
        println!("odd {}", n);
    }
    // Result
    // odd 1
    // odd 3

    // break: 반복 중단
    for n in 0..5 {
        if n % 2 == 1 {
            break;
        }
        println!("even {}", n);
    }
    // Result
    // even 0

    // break: 루프 라벨에서 벗어나기
    // 라벨은 (`)를 식별자를 붙혀 선언함
    'outer: for x in 0.. {
        for y in 0.. {
            for z in 0.. {
                if x + y + z > 1000 {
                    break 'outer;
                }
            }
        }
    }
}

// while보다 왠만함 loop를 써라.
pub fn loop_the_basis_for_rusts_looping_constructs() {
    // 루프는 다음 예제와 같이 장기 실행 서버를 구현할 때 자주 나타납니다.
    loop {
        // let requester, request = accept_request();
        // let result = process_request(request);
        // send_response(requester, result);
    }
}

// Rust는 표현 기반의 언어다
pub fn rust_is_an_expression_based_language() {
    let n = 123456;

    // if
    // if 안의 마지막 표현이 return 됨
    let description = if is_even(n) { "even" } else { "odd" };
    println!("{} is {}", n, description);

    // match
    let description_match = match is_even(n) {
        true => "even",
        false => "odd",
    };
    println!("matched: {} is {}", n, description_match);

    let n = loop {
        // break도 값을 반환함
        break 123;
    };

    println!("{}", n);

    let needle = 42;
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        // _ => 는 default: 경우를 뜻함
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };

        if result == "hit!" {
            println!("{}: {}", item, result);
        }
    }
}

fn is_even(n: i32) -> bool {
    n % 2 == 0 // 마지막 표현이 return 됨
}

// Generic
// 그냥 T로 받으면 +가 있는지 알 수 없기 때문에, Add Trait을 구현하고 있는 애들만 받아야함.
fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

pub fn a_generic_function_with_a_type_variable_and_trait_bounds() {
    let floats = add(1.2, 3.4);
    let ints = add(10, 20);
    let durations = add(Duration::new(5, 0), Duration::new(10, 0));

    println!("floats {}", floats);
    println!("ints {}", ints);
    println!("durations {:?}", durations);
}
/* Result
floats 4.6
ints 30
durations 15s
*/

// Lifetime parameters
/*
i에 `a의 라이프 사이클을 바인딩 한다.
j에 `b의 라이프 사이클을 바인딩 한다.
*/
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

fn lifetime() {
    let a = 10;
    let b = 20;
    let res = add_with_lifetimes(&a, &b);

    println!("{}", res);
}
