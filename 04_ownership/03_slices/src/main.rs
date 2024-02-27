fn main() {
    /* let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid! */

    // Use a slice, a reference to a range
    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    let world = &s[..]; // SLice of whole String!

    // Careful that slices assume ASCII chars (no multibyte chars allowed!)

    // String literals are immutable slices
    let s_literal = "Hello, world!";

    // This is true for any kind of slice like array slices
    // Slices store first element and length
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// We can return a slice!
// The compiler will check the validity of the reference
fn second_half(s: &str) -> &str { // taking a &str allows &String too
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i + 1..];
        }
    }

    &s[..]
}
