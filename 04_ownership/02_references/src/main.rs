fn main() {
    let s1 = String::from("hello");

    // Note that we have to use `&` here even though it is
    // already marked as reference in the function declaration
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    /* // &mut allows changing a variable, only one can be created at a time (ownership)
    let mut s = String::from("hello");
    //{ <- This scope makes sure we have only one reference at a time
    let r1 = &mut s;

    //  }
    let r2 = &mut s;
    println!("{r1} {r2}") */

    // Also can't have a mutable and an immutable reference

    // NOTE: A reference’s scope starts from where it is introduced and continues through the last time that reference is used
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

/* // This will not compile, it is borrowing:
fn change(some_string: &String) {
    some_string.push_str(", world");
} */

/* // References can't be dangling
fn dangle() -> &String {
    let s = String::from("hello");

    &s
} */
