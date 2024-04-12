fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// `fn` works with both closures and functions. Only functions can work with external code like C

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
