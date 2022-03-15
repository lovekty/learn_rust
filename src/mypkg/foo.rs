pub mod foo1 {
    pub fn foo1_fn1() {
        println!("foo1 fn1");
    }
}

pub mod foo2 {
    pub fn foo2_fn1() {
        super::foo1::foo1_fn1();
        crate::mypkg::bar::bar_fn1();
    }
}

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
