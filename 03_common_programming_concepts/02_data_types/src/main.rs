use std::io;

fn main() {
    // Type inference similar to c++
    /* let guess: u32 = "42".parse().expect("Not a number!"); */

    // isize and usize depends on architecture (32/64) bits

    // Can use '_' as visual separator for integers!

    // Rust panics in debug mode when integer overflows happen

    // float defaults to f64
    /* let x = 2.0; // f64 */

    /* let y: f32 = 3.0; // f32 */

    // Operations and bools have nothing weird. Chars can be emoji or glyphs too.
    // char size is 4 bytes, 1 in c++ I think?
    /* let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻'; */

    // Tuples
    /* let tup: (i32, f64, u8) = (500, 6.4, 1); // Types are optional */
    /* let tup = (500, 6.4, 1); */
    /* let (x, y, z) = tup; // destructuring is supported */

    // NOTE: can access the tuple with the '.' operator and index
    /* let y = tup.1; */

    // A unit is a tuple without any value `()` which expression implicitly return if the don't
    // return a value

    // Arrays
    let a = [1, 2, 3, 4, 5];
    /* let allThrees = [3; 5]; // five `3`s array initialization */

    // Invalid access compile time
    /* a[6]; // This is caught at compile time! (nice) */

    // Invalid access runtime, panics (even in release mode!)
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
