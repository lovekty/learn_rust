use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub struct Foo {
    pub foo: i32,
    pub bar: i32,
}

impl Add<&Foo> for Foo {
    type Output = Self;

    fn add(self, rhs: &Foo) -> Self {
        Self {
            foo: self.foo + rhs.foo,
            bar: self.bar + rhs.bar,
        }
    }
}

impl Add<Foo> for Foo {
    type Output = Self;

    fn add(self, rhs: Foo) -> Self {
        Self {
            foo: self.foo + rhs.foo,
            bar: self.bar + rhs.bar,
        }
    }
}

impl Display for Foo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Foo(foo:{}, bar:{})", self.foo, self.bar)
    }
}
