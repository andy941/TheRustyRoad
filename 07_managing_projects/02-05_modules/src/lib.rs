mod front_of_house {
    pub mod hosting {
        // fn add_to_waitlist() {} // Wrong
        pub fn add_to_waitlist() {} // Correct

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // Reference a function in upper module
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Public enums have all variants public
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast); // Public member

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

// `use` should bring modules into scope and full paths to structs or enums, that is the
// convention ...

// You can provide you names with the `use` keyword

// Combine pub and use to have the `use` defined available to callers of the module
pub use crate::front_of_house::hosting;

pub fn eat_at() {
    hosting::add_to_waitlist();
}

// Nested use
use std::{cmp::Ordering, io};

// Bring all public items into scope from module
use std::collections::*;
