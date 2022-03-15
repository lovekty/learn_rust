use std::fmt::Display;

pub enum MyResult<T, E> {
    Ok,
    OkWithPayload(T),
    Pending { waiting: u64, time_unit: String },
    Err(E),
}

pub fn process_my_result<T: Display, E>(data: MyResult<T, E>) {
    match data {
        MyResult::Ok => {
            println!("ok");
        }
        MyResult::OkWithPayload(t) => {
            println!("ok with payload: {}", t);
        }
        MyResult::Pending { waiting, time_unit } => {
            println!("wait for {} {}", waiting, time_unit);
        }
        MyResult::Err(e) => {
            println!("error")
        }
    }
}
