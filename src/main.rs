mod basic_knowledge;
mod branch;
mod guess_number;
mod ownership_demos;
mod structs_demos;

fn main() {
    // let rec = structs_demos::Rectangle {
    //     length: 10,
    //     width: 5,
    // };
    // println!(
    //     "rectangle: {:#?} with area:{}",
    //     &rec,
    //     structs_demos::area(&rec)
    // );

    let rec2 = structs_demos::RectangleV2 {
        length: 10,
        width: 5,
    };
    println!("{} with area:{}", &rec2, rec2.area())
}
