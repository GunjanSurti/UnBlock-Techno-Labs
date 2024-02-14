use chapter_17_oops::{Button, Draw, Screen};
#[allow(dead_code)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn main() {
    #[allow(unused_variables)]
    let screen = Screen {
        // we can use SelectBox and Button bcz they implement Draw trait
        components: vec![
            Box::new(SelectBox {
                width: 100,
                height: 100,
                options: vec![
                    String::from("yes"),
                    String::from("no"),
                    String::from("maybe"),
                ],
            }),
            Box::new(Button {
                width: 100,
                height: 100,
                lable: String::from("Ok"),
            }),
        ],
    };
}

/*
*********************Trait Objects Perform Dynamic Dispatch*******************

"monomorphization" process performed by the compiler when we use trait bounds on generics: the compiler generates "nongeneric implementations" of functions and methods for each concrete type that we use in place of a generic type parameter.

The code that results from monomorphization is doing static dispatch, which is when the compiler knows what method you’re calling at compile time.

This is opposed to "dynamic dispatch" => which is when the compiler can’t tell at compile time which method you’re calling. In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.

"When we use trait objects, Rust must use dynamic dispatch."

The compiler doesn’t know all the types that might be used with the code that’s using trait objects, so it doesn’t know which method implemented on which type to call. Instead, at "runtime", Rust uses the "pointers inside the trait object" to know which method to call.

This lookup "incurs a runtime cost" that doesn’t occur with static dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations. However, we did get extra flexibility in the code that we wrote in Listing 17-5 and were able to support in Listing 17-9, so it’s a trade-off to consider.


*/
