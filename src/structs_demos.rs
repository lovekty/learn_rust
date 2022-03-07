use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Rectangle {
    pub length: u32,
    pub width: u32,
}

pub fn area(rec: &Rectangle) -> u32 {
    rec.length * rec.width
}

#[derive(Debug)]
pub struct RectangleV2 {
    pub length: u32,
    pub width: u32,
}

impl RectangleV2 {
    pub fn area(&self) -> u32 {
        self.length * self.width
    }
}

impl Display for RectangleV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RectangleV2(length:{} width:{})",
            self.length, self.width
        )
    }
}
