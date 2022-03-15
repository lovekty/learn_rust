pub trait MyTrait1 {
    fn hello(&self);
}

pub trait MyTrait2 {
    fn hello(&self);
}

pub struct MyFoo {}

impl MyTrait1 for MyFoo {
    fn hello(&self) {
        println!("trait1")
    }
}

impl MyTrait2 for MyFoo {
    fn hello(&self) {
        println!("trait2")
    }
}

pub fn process(t: &impl MyTrait1) {}
