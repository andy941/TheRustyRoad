#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:#?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn main() {
    let coin = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("Value coin: {coin}");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Number = {six:?}!");
    println!("Number = {none:?}!");

    // `other` will match every case,  beware to put it last or Rust will not run the remaining
    // cases
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other),
        // _ => reroll(), // like the `other` above but without passing a value
        _ => () // Unit-value allows to do nothing
    }
}
