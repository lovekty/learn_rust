pub fn bar_fn1() {
    super::foo::foo1::foo1_fn1();
}

use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}
