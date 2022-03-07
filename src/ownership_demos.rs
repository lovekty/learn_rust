pub mod scope_demos {
    pub fn str_trick() {
        let mut s = String::from("hello");
        // let mut s= String::new();
        // s.push_str("hello");
        s.push_str(", world!");
        println!("{}", s);
    }

    pub fn str_reassign() {
        let s1 = String::from("hello");
        let s2 = s1;
        // invalid for compiler
        // println!("{}", s1);
        println!("{}", s2);
    }

    pub fn str_clone() {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1:{}\ns2:{}", s1, s2);
    }

    pub fn i32_reassign() {
        let num1 = 10;
        let mut num2 = num1;
        num2 += 1;
        // ok cause i32 in stack
        println!("num1 is:{}\nnum2 is:{}", num1, num2);
    }

    pub fn show_ownership() {
        let s = String::from("hello");
        take_ownership(s);
        // println!("{}", s);

        let x = 5;
        makes_copy(x);
        println!("{}", x);
    }

    fn take_ownership(string: String) {
        println!("{}", string);
    }

    fn makes_copy(num: i32) {
        println!("{}", num);
    }
}

pub mod reference_demos {
    pub fn trick() {
        let mut s = String::from("hello");
        change_something(&mut s);
        // change_something(&mut s);// this is ok
        // change_something2(&mut s, &mut s); // invalid
        println!("{}", s);
    }

    fn change_something(string: &mut String) {
        string.push_str(", world");
    }

    fn change_something2(string1: &mut String, string2: &mut String) {
        string1.push_str(", world");
        string2.push_str(", world");
    }

    pub fn no_dangling() -> String {
        let s = String::from("hello");
        return s;
    }
}

pub mod slice_demos {
    pub fn first_word(string: &str) -> &str {
        for (i, &item) in string.as_bytes().iter().enumerate() {
            if item == b' ' {
                return &string[0..i];
            }
        }
        string
    }

    pub fn top_ten<T>(elements: &[T]) -> &[T] {
        if elements.len() > 10 {
            return &elements[0..10];
        }
        elements
    }

    pub fn use_top_ten() {
        let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
        let top_ten = top_ten(&nums);
        for item in top_ten {
            print!("{} ", item);
        }
        println!()
    }
}
