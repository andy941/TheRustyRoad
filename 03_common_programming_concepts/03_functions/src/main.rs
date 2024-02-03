fn another_function(x: i32 /* must declare the type, always  */) {
    println!("Another function value is {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1 // compile error with `;`
}

fn main() {
    // SnakeCase convention for functions
    println!("Hello, world!");
    another_function(0b0010);

    // Statementes do not return values, can't do:
    // x = y = 6

    // NOTE: any scope block created with curly braces is an expression IF THE LAST LINE DOES NOT
    // HAVE A SEMICOLON!
    // Also, the return calie is sinonimous with the final expression of the body of a function
    /* let z = five(); */
    /* let z = plus_one(2); */
}
