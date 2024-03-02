use std::option;

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}
/* struct IpAddr {
    kind: IpAddrKind,
    address: String,
} */

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// Great thing about enums is that they can hold a variety of types and we can implement methods
// for them too
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

// Rust has no concept of NULL values or references
enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    /* let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    }; */

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    // &slef would be String
    let m = Message::Write(String::from("hello"));
    m.call();

    // Option::
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: option::Option<i32> = None;

    /* // Does NOT compile
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y; */
}
