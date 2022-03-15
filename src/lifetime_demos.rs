fn longest<'a, 'b: 'a>(str1: &'a str, str2: &'b str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

#[test]
pub fn use_longest() {
    let str1 = String::from("helloworld");
    // let res;
    {
        let str2 = String::from("haha");
        let res = longest(str2.as_str(), str1.as_str());
        println!("result is {}", res);
        //res = longest(str1.as_str(), str2.as_str());
    }
    // println!("result is {}", res);
}
