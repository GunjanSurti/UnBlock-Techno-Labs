use std::fmt::Display;

pub fn struct_lifetime() {
    /****************************Lifetime Annotations in Struct Definitions******************************************/
    let i = Important {
        part: "This is struct Lifetime",
    };
    // println!("{:#?}",i);

    // The Static Lifetime
    // One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the entire duration of the program. All string literals have the 'static lifetime, which we can annotate as follows:

    let s: &'static str = "I have a static lifetime.";
    // The text of this string is "stored directly in the program’s binary", which is always available. Therefore, the lifetime of all string literals is 'static.

    // IMPORTANT
    // You might see suggestions to use the 'static lifetime in error messages. But before specifying 'static as the lifetime for a reference, think about whether the reference you have actually lives the entire lifetime of your program or not. Most of the time, an error message suggesting the 'static lifetime results from attempting to create a dangling reference or a mismatch of the available lifetimes. In such cases, the solution is fixing those problems, not specifying the 'static lifetime.
}

fn longest_function<'a, T>(x: &'a str, ann: T, y: &'a str) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct Important<'a> {
    part: &'a str,
}
impl<'a> Important<'a> {
    fn level(&self) -> i32 {
        // no need for lifetime bcz of 3rd rule
        3
    }
}

/****************************************************
* Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.
* compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations
* 1st applies to input lifetimes
* 2nd and 3rd => output lifetimes
*
* 1st => The first rule is that the "compiler assigns a lifetime parameter" to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter
* eg fn foo<'a,'b> (x: &'a i32, y: &'b i32), two diff para, 2 diff lifetime
* 2nd => if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters:
*3rd => if there are multiple input lifetime parameters, but one of them is "&self or &mut self" because this is a "method", the lifetime of self is assigned to all output lifetime parameters



*/
