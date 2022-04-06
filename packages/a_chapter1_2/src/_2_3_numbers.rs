/*
알파벳 뒤 숫자는 bit. 메모리도 그만큼 먹음.

i: Signed integers, 부호 있는 정수
i8, i16, i32, i64

u: Unsigned integers, 부호 없는 정수
u8, u16, u32, u64

f: Floating-point types, 부동 소수점
f32, f64

isize, usize: 32bit 컴퓨터의 경우 32, 64bit 컴퓨터의 경우 64,
*/

// 숫자 유형
pub fn numeric_literals_and_basic_operations_on_numbers_in_rust() {
    // 3가지 numeric 타입 정의 방법
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;

    let addition = twenty + twenty_one + twenty_two;
    println!(
        "{} + {} + {} = {}",
        twenty, twenty_one, twenty_two, addition
    );

    let one_million: i64 = 1_000_000; // _ 구분자로 사용 가능
    println!("{}", one_million.pow(2));

    let forty_twos = [42.3424, 42f32, 42.3423_f32];

    println!("{:02}", forty_twos[0]); // {:02} 가 무얼 의미?
}
/* Result
20 + 21 + 22 = 63
00000000000
42.3424
*/

// 진수
pub fn using_base_2_base_8_and_base_16_numeric_literals() {
    let three = 0b11; // b: 2진수
    let thirty = 0o36; // o: 8진수
    let three_hundred = 0x12C; // x: 16진수

    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2:  {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8:  {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}
/* Result
base 10: 3 30 300
base 2:  11 11110 100101100
base 8:  3 36 454
base 16: 3 1e 12c
*/

// 부동소수점 위험
/*
! 비교연산을 피하라

아래 f32는 성공하고 f64는 실패하는 이유는
수학적 연산이 실제 수학적 결과의 허용 가능한 범위 내에 있지 않기 때문이다.
방어적으로 프로그래밍하려면 is_nan() 및 is_finite() 메서드를 사용합니다. 수학적 오류를 자동으로 진행하지 않고 충돌을 유도하면 문제의 원인을 디버깅할 수 있습니다.
다음은 is_finite() 메서드를 사용하여 이 상태를 발생시키는 방법을 보여 줍니다.
*/
pub fn floating_point_hazards() {
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert_eq!(abc.0 + abc.1, abc.2); // success
    assert_eq!(xyz.0 + xyz.1, xyz.2); // failed
                                      /*
                                          left: `0.30000000000000004`,
                                          right: `0.3`'
                                      */

    let x: f32 = 1.0 / 0.0;
    assert!(x.is_finite()); // failed
}
/* Result
abc (f32)
   0.1 + 0.2: 3e99999a
         0.3: 3e99999a

xyz (f64)
   0.1 + 0.2: 3fd3333333333334
         0.3: 3fd3333333333333
*/
