type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
}

/* // Nevertype pattern
fn bar() -> ! {
    // --snip--
} */

/*
fn generic<T>(t: T) {
    // --snip--
}

// This is the long form of the above
fn generic<T: Sized>(t: T) {
    // --snip--
}
*/

// The `?` means dynamycally sized or not known at compile time. Notice `t` is now a reference
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}

fn main() {
    type Kilometers = i32; // like `using` in C++, use to shorten long types

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}
