fn main() {
    let mut s = String::new();
    // `to_string()` is available to any type that implements the Display trait.
    let data = "initial contents";
    let s = data.to_string();
    let s = String::from(data); // Equivalent to above

    // Remember:  strings are UTF-8 encoded
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str(" bar");
    s.push('l');

    // Adding strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used while
                       // let s3 = format!("{s1}{s2}");  // This uses only references!

    // STRINGS in Rust do NOT allow INDEXING!
    // Because well UTF-8 formatting ...

    // Iterate over chars
    for c in "Зд".chars() {
        println!("{c}");
    }
    // Iterate over bytes
    for c in "Зд".bytes() {
        println!("{c}");
    }

    // Grapheme clusters are not implemented in the standard library
}
