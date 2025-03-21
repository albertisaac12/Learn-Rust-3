// mod front_of_the_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }
//===========================================================================
// mod eat_at_rest {
// pub mod front_of_house {
//    pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// mod dining {
//     pub fn eat_at_restaurant() {
//         // Absolute path
//         crate::eat_at_rest::front_of_house::hosting::add_to_waitlist();

//         // Relative path
//         super::front_of_house::hosting::add_to_waitlist();
//     }
// }
// }

// fn deliver_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }

//     fn cook_order() {}
// }

//===========================================================================
// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     // Order a breakfast in the summer with Rye toast
//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     // Change our mind about what bread we'd like
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);

//     // The next line won't compile if we uncomment it; we're not allowed
//     // to see or modify the seasonal fruit that comes with the meal
//     // meal.seasonal_fruit = String::from("blueberries");
// }
//==================================

// mod back_of_house {
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_restaurant() {
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }

//=======================================================================
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }


// use crate::front_of_house::hosting;

// mod customer {
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist(); // will throw error
//     }
// }
//===========================================================================

/* example problem */
// mod front_of_house;
// mod abc {
    
// }

//========================================================================
// too much work
// use std::io;
// use std::cmp::Ordering;

// solution
// use std::{io,cmp::Ordering};


// problem
// use std::io;
// use std::io::Write;

// solution
// use std::io::{self,Write};

//===========================================================================

// mod front_of_house;

// use front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist();
// }

//===========================================================================

// this will be my main library module

// catalog module // add and remove book
// services module // borrow and return book
//users module // register book



// mod library  {

//     const users:[i32;5] = [3; 5];

//     mod catalog{

//     }
//     mod services{

//     }
//     mod users{

//     }

// }

// ============================================================================================================

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}