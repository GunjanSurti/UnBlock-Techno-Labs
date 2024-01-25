#![allow(dead_code)]
mod lib_;
mod front_of_house {
    // it is "not public" but front_of_house and eat_at_restaurant are "siblings" we can refer thsi function from eat_at_restaurant
    // "crate", the root of our crate’s module tree
    // everything is private by default so make pub to use them
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}

fn deliver_order() {}
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
        //  we can use "super" to go to the parent module of back_of_house, which in this case is "crate", the root
        //  we assume that deliver_order and back_of_house stays in same relationship
    }
    fn cook_order() {}
}
// This is library "cargo new Chapter_7_2_restaurant_library --lib"

// Modules can also hold definitions for other items, such as structs, enums, constants, traits, and—as in Listing 7-1—functions.

/*
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
*/

/*
A path can take two forms:

An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
A relative path starts from the current module and uses self, super, or an identifier in the current module.
Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).
*/

/*
Adding the "pub" keyword in front of "mod hosting" makes the module "public". With this change, if we can access front_of_house, we can access hosting. But the "contents of hosting are still private"; making the module public doesn’t make its contents public. The pub keyword on a module only lets code in its ancestor modules refer to it, not access its inner code. Because modules are containers, there’s not much we can do by only making the module public; we need to go further and choose to make one or more of the items within the module public as well.
*/

/*
In the "absolute path", we start with crate, the root of our crate’s module tree. The front_of_house module is defined in the crate root. While front_of_house isn’t public, because the eat_at_restaurant function is defined in the same module as front_of_house (that is, eat_at_restaurant and front_of_house are siblings), we can refer to front_of_house from eat_at_restaurant. Next is the hosting module marked with pub. We can access the parent module of hosting, so we can access hosting. Finally, the add_to_waitlist function is marked with pub and we can access its parent module, so this function call works!

In the "relative path", the logic is the same as the absolute path except for the first step: "rather than starting from the crate root, the path starts from front_of_house". The front_of_house module is defined within the same module as eat_at_restaurant, "so the relative path starting from the module in which eat_at_restaurant is defined works". Then, because hosting and add_to_waitlist are marked with pub, the rest of the path works, and this function call is valid!

*/

// Using "super" allows us to reference an item that we know is in the parent module, which can make rearranging the module tree easier when the module is closely related to the parent, but the parent might be moved elsewhere in the module tree someday.
mod front_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
