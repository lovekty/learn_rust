use crate::generic_demos::traits_demos::{MyTrait1, MyTrait2};
use std::fs;
use std::fs::File;
use std::io::{self, Read};

mod basic_knowledge;
mod branch;
mod collections_demos;
mod enums_demos;
mod generic_demos;
mod guess_number;
mod lifetime_demos;
mod mypkg;
mod ownership_demos;
mod structs_demos;

fn main() {
    // generic_demos::use_myobj();
    let foo = generic_demos::traits_demos::MyFoo {};
    MyTrait1::hello(&foo);
    MyTrait2::hello(&foo)
}

fn fn1() {
    fn2()
}

fn fn2() {
    fn3()
}

fn fn3() {
    panic!("crash!")
}

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("")
}
