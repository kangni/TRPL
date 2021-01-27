mod front_of_house;

use std::fmt::Result;
use std::io::Result as IoResult;
// use std::{cmp::Ordering, io};
use std::io::{self, Write};

// for tests
// use std::collections::*;

// fn func1() -> Result {}

// fn func2() -> IoResult<T> {}


fn serve_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

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

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}


use front_of_house::hosting;
// use front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    // order a breakfast in the summer with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change out mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    // add_to_waitlist();

    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}
