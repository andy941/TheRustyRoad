fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // Other fields should be the same
    }; // Also order does not matter!
       // Careful that now user1 is not available anymore, username string was moved!
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // no need to specify if member and variable have the same name
        email,
        sign_in_count: 1,
    }
}

// Tuple structs, can destructure with `.<index>`
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct, useful when needing to implement a trait, acts similar to ()
struct AlwaysEqual;

/* // This won't work, does not own the data. Needs lifetimes to work, to make sure data is kept alive
// for the lifetime of the struct
struct UserB {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
} */
