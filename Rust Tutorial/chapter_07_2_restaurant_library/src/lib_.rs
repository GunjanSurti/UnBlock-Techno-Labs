#![allow(dead_code)]
#![allow(unused_imports)]
// If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private
mod back_house {
    pub struct Breakfast {
        // default is private but made public
        pub toast: String,      // default is private but made public
        seasonal_fruit: String, // default is private so cannot change directly
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_house::Breakfast::summer("Brt");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Art");

    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal

    // meal.seasonal_fruit = String::from("blueberries");
}
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
pub fn eat_at_restaurant() {
    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

// so the default for enum variants is to be public. Structs are often useful without their fields being public, so struct fields follow the general rule of everything being private by default unless annotated with pub.

/*******************************************************************************************/

// Having to write out the paths to call functions can feel inconvenient and repetitive
// we can create a shortcut to a path with the "use" keyword once, and then use the shorter name everywhere else in the scope.

// mod front_house { this code is written in lib.rs
//     pub mod hosting {
//         pub fn add_to_waitlist(){}
//     }
// }
// crate => root file here
use crate::front_house::hosting;
// importing from froot file (lib.rs)
// By adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in that scope, just as though the hosting module had been defined in the crate root. Paths brought into scope with use also check privacy, like any other paths.
pub fn eat_at() {
    // can use hosting bcz this is not module so it can access
    hosting::add_to_waitlist();
    // here we can import "use crate::front_house::hosting::add_to_waitlist()" but we didnt do that
    // bcz add_to_waitlist is function of different module so we want to differentiate from this files function
}

mod customer {
    use crate::front_house::hosting;
    // we have to import here even though we already have in above lines bcz
    // we are in different module
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

// On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path. Listing 7-14 shows the idiomatic way to bring the standard library’s "HashMap" struct into the scope of a binary crate.

use std::collections::HashMap; // struct "HashMap"
fn map() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// The "exception" to this idiom is if we’re bringing "two" items with the "same name into scope with use statements", because Rust doesn’t allow that. Listing 7-15 shows how to bring two Result types into scope that have the same name but different parent modules and how to refer to them.

// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// use std::fmt::Result;
// use std::io::Result as IoResult; giving name using "as" keyword

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

// Members of the Rust community have made many packages available at crates.io, and pulling any of them into your package involves these same steps:" listing them in your package’s Cargo.toml file and using use to bring items from their crates into scope."

// use std::cmp::Ordering;
// use std::io;
// here we are using same package so...
use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;
// use std::io::{self,Write};

// If we want to bring "all public items" defined in a path into scope, we can specify that path followed by the 
// " * glob operator"

use std::collections::*;
