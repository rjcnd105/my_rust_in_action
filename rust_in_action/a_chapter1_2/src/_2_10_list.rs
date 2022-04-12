/*
Array:  배열 내의 데이터는 수정할 수 있지만, 크기는 조정 불가능. [T, n]

Slice:  Array like object인데 크기가 동적임. 그렇기 때문에 동적 타이핑이 됨. [T]
        Slice는 slice에 대한 trait을 더 구현하기 쉬움
        읽기 권한을 빠르게 얻어올 수 있으므로 Array나 다른 slice에 대한 view 역할을 기능을 하기도 함.
        크기가 동적이므로 일반적으로 &[T]로 참조 접근해서 사용함.

Vector: 확장 가능한 목록. 크기가 자유롭게 변경될 수 있기에 성능적 런타임 패널티가 발생.
*/
pub fn defining_arrays_and_iterating_over_their_elements() {
    let one = [1, 2, 3];
    // u8가 3개인 array을 의미
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3]; // ;은 0으로 3개를 채운다는 뜻. 햇갈리네
    let blank2: [u8; 3] = [0; 3];

    let arrays = [one, two, blank1, blank2];

    for item in &arrays {
        print!("item: {:?}: ", item);
        for n in item.iter() {
            print!("\t{} + 10 = {},", n, n + 10);
        }

        let mut sum = 0;
        for i in 0..item.len() {
            sum += item[i];
        }
        println!("\t({:?} = {})", item, sum);
    }
}
