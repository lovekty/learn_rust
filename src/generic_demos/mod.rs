mod mixup;
pub mod traits_demos;

use std::fmt::Display;

struct MyObj<T> {
    pub hold: T,
}

struct Foo {}

impl MyObj<i32> {
    pub fn process_int(&self) -> i32 {
        self.hold * 2
    }
}

impl<T: Display> MyObj<T> {
    pub fn process_display(&self) -> String {
        format!("hold:{}", self.hold)
    }
}

impl<T> MyObj<T> {
    pub fn process_hold(&self) -> String {
        String::from("process hold")
    }
}

pub fn use_myobj() {
    let mo1 = MyObj { hold: 12 };
    println!("{}", mo1.process_int());

    let mo2 = MyObj {
        hold: String::from("hello world"),
    };
    println!("{}", mo2.process_display());

    let mo3 = MyObj { hold: Foo {} };
    println!("{}", mo3.process_hold());
}
