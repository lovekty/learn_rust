pub fn func(count: u32, unit_label: char) {
    println!("we have {}{}", count, unit_label);
}

pub fn exp() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("y is {}", y);
}

pub fn plus_one(num: i32) -> i32 {
    // num + 1
    return num + 1;
}
