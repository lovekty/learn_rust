pub fn if_demo() {
    let x = 3;
    if x > 5 {
        println!("greater than 5");
    } else if x > 0 {
        println!("greater than 0 but not than 5");
    } else {
        println!("not greater than 0")
    }
}

pub fn if_exp_demo() {
    let x = 10;
    let y = if x > 0 { "positive" } else { "negative" };
}
