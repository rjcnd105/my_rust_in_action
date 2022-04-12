struct Demo {
    a: i32,
}

fn use_value(_val: i32) {}

fn use_demo_value(_val: Demo) {}

/// primitive value들은 Copy trait을 구현하고 있기 때문에
/// move 되지 않고 copy된다.
/// 주의하라! 이것이 기본적인 케이스가 아니라 특별한 케이스이다!
pub fn primitive_value_is_copied_rather_than_moved() {
    let a = 123;
    use_value(a);

    println!("coped: {}", a);


    let demo = Demo { a: 123 };
    use_demo_value(demo);

    // Error! value borrowed here after move
    // demo는 이미 use_demo_value가 빌려가버려서 엑세스할 수 없다.
    // println!("{}", demo.a)
}

