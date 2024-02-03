fn main() {
    /* let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}"); */

    /* const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; */

    // Interesting difference with c++:
    // YOu can shadow variables. When shadowing a variabel you can also CHANGE THE TYPE.
    /* let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}"); */

    // Can be more readable (?)
    /* let spaces = "   ";
    let spaces = spaces.len(); */

    // Still won't compile this (can't change the type of a mut, ONLY SHADOW A CONST)
    let mut spaces = "   ";
    spaces = spaces.len();
}
