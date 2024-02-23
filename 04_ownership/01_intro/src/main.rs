fn main() {
    let mut s = String::from("hello");
    /* let s2 = s; */
    /* let s2 = s.clone(); */
    s.push_str(", world!");

    println!("{}", s);

    // Rust always defaults to shallow copy
    // You can add the Copy trait to a type to make it trivially copyable but NOT if it already has
    // the Drop trait.

    // Rust can return multiple values with a tuple
    let (s2, len) = calculate_length(s);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
