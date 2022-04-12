use std::cell::RefCell;
use std::rc::Rc;
use crate::cube_sat::{GroundStation, Mailbox, Message};

mod cube_sat;
mod primitive_values;

fn main() {
    // ownership_return_test();
    // primitive_values::primitive_value_is_copied_rather_than_moved();
    // sat_message_step1();
    // sat_message_step2();
    sat_message_step3();
}

// 서버에서 sat ids를 받아온다 가정한 함수.
fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn sat_message_step3() {
    // RC: Reference Counted
    // Rc<T>는 mutation을 허용하지 않기 때문에, RefCell로 감싸줘야만 가변적이게 된다.
    // Rc<T>는 스레드로부터 안전하지 않다.
    // 다중 스레드에서는 Arc<T>로, 가변은 Arc<Mutex<T>>로 바꾸는 것이 훨신 좋다.
    // Arc: Atomic reference counter.
    let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(
        GroundStation {
            radio_freq: 87.65
        }
    ));

    println!("base: {:?}", base);
    // base: RefCell { value: GroundStation { radio_freq: 87.65 } }

    // 빌려주는 스코프를 지정
    {
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("base_2: {:?}", base_2);
        // base_2: GroundStation { radio_freq: 75.31 }
    }

    println!("base: {:?}", base);
    // base: RefCell { value: GroundStation { radio_freq: 75.31 } }

    let mut base_3 = base.borrow_mut();
    base_3.radio_freq += 43.21;

    println!("base: {:?}", base);
    // base: RefCell { value: <borrowed> }
    println!("base_3: {:?}", base_3);
    // base_3: GroundStation { radio_freq: 118.52000000000001 }
}

// fn sat_message_step2() {
//     let mut mail = Mailbox { messages: vec![] };
//
//     // Rc<T>는 mutation을 허용하지 않기 때문에, RefCell로 감싸줘야만 가변적이게 된다.
//     let base: GroundStation {}
//     let sat_ids = fetch_sat_ids();
//
//     for sat_id in sat_ids {
//         let sat = base.connect(sat_id);
//         let msg = Message { to: sat_id, content: String::from("hello") };
//         base.send(&mut mail, msg);
//     }
//
//     let sat_ids = fetch_sat_ids();
//
//     for sat_id in sat_ids {
//         let sat = base.connect(sat_id);
//
//         let msg = sat.recv(&mut mail);
//         println!("{:?}: {:?}", sat, msg);
//     }
// }

// fn sat_message_step1() {
//     let base = GroundStation {};
//     let mut sat_a = CubeSat {
//         id: 0,
//         mailbox: Mailbox {
//             messages: vec![]
//         },
//     };
//
//     println!("t0: {:?}", sat_a);
//
//     base.send(&mut sat_a, Message::from("hello there!"));
//
//     println!("t1: {:?}", sat_a);
//
//     let msg = sat_a.recv();
//
//
//     println!("t2: {:?}", sat_a);
//
//     println!("msg: {:?}", msg);
// }


// fn ownership_return_test() {
//     let sat_a = cube_sat::CubeSat { id: 0, mailbox: Mailbox { messages: vec![] } };
//     let sat_b = cube_sat::CubeSat { id: 1, mailbox: Mailbox { messages: vec![] } };
//     let sat_c = cube_sat::CubeSat { id: 2, mailbox: Mailbox { messages: vec![] } };
//
//     let a_status = check_status(sat_a);
//     let b_status = check_status(sat_b);
//     let c_status = check_status(sat_c);
//     println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
//
//     // "waiting" ...
//     let a_status = check_status(a_status);
//     let b_status = check_status(b_status);
//     let c_status = check_status(c_status);
//     println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
// }
//
// fn check_status(cube_sat: CubeSat) -> CubeSat {
//     println!("{}, {:?}", cube_sat.id, StateMessage::Ok);
//     cube_sat
// }
