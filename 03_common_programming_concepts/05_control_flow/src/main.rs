use std::io;

fn main() {
    /* let number = 7; */

    // NOTE: no conversion from int to bool!
    /* if number < 5 {
        println!("false");
    } else {
        println!("true {number}");
    } */

    // More involved if-else
    // Probably better to use `match`
    /* if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    } */

    /* let condition = true;
    let mut number = if condition { 5 } else { 6 };

    // Infinite loop, notice that `break` and the loop can return a value!
    let result = loop {
        println!("Again!");
        number = number - 1;
        if number == 0 {
            break number * 2;
        }
    };

    println!("Result = {result}"); */

    // Can specify loop labels, can exit the outer loop in one go
    /* let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}"); */

    // NOTE: Can loop over collections by index but the compiler WILL ADD A RUNTIME CHECK WITH A
    // PERFORMANCE HIT
    // That does not happen with range-based loops
    /* let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // Or with ranges
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!"); */

    /* loop {
        println!("Please enter a temperature.");
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");
        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That is not a number");
                continue;
            }
        };
        let cels: f64 = far_cels(temp);
        println!("The farheneit temperature {temp} is {cels} celsius");
        break;
    } */

    loop {
        println!("Please enter a positive number.");
        let mut num = String::new();
        io::stdin()
            .read_line(&mut num)
            .expect("Failed to read line");
        let num: usize = match num.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That is not a number");
                continue;
            }
        };
        let fib = fibonacci_nth(num);
        println!("Fibonacci number {num} is {fib}");
    }
}

/* fn far_cels(x: f64) -> f64 {
    (x - 32.0) * 5.0 / 9.0
} */

fn fibonacci_nth(x: usize) -> usize {
    if x == 0 {
        return 0;
    }
    let mut prev: usize = 0;
    let mut next: usize = 1;
    let mut result: usize = 1;
    for _ in 1..x {
        result = prev + next;
        prev = next;
        next = result;
    }
    return result;
}
