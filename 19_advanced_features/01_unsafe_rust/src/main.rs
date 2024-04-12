unsafe fn dangerous() {}

use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // We have to use this otherwise Rust would think we are borrowing from the same slice when in
    // fact we are borrowing non-overlapping slices
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Calling functions from other languages is inherently unsafe
extern "C" {
    fn abs(input: i32) -> i32;
}

// Make function accessible from C code after compiling into shared library
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Global variables are called `static` variables in Rust
static HELLO_WORLD: &str = "Hello, world!"; // NOTE: modifying mutable static variables is unsafe

// Traits can be unsafe too
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {
    let mut num = 5;

    // Raw pointers can be created without the need of an `unsafe` block
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Calling functions from other languages is inherently unsafe
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("name is: {}", HELLO_WORLD);
}
