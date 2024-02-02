mod struct_lifetime;
fn main() {
    // let string1 = String::from("abcd");
    // let string2 = "xyz";

    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);
struct_lifetime::struct_lifetime();
}
#[allow(dead_code)]
/********************************Lifetime Annotation in Function Signatures**************************************/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // here the life time of refrence retruned is same as shortest lifetime of input variable
    /* using " ' " tick or apostrophe name can be lower case and usually smalle eg 'a, 'apple etc */

    if x.len() > y.len() {
        x
    } else {
        y
    }
    // When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y. In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y. Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.
}

/***************************************************************************************************************/
// fn longest(x: &str, y: &str) -> &str { "missing lifetime specifier"
// this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`, means here lifetime is not decided for return value so it shows error
// bcz x or y dont have fix lifetime
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
/***************************************The Borrow Checker**************************
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
*/

/********************************Lifetime Annotation Syntax*************************************

Lifetime annotations don’t change how long any of the references live.

"IMP" =>they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime

we’ll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis.

when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints

/***********************************************************************************************/

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
We’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.

When "returning a reference" from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters. 
If the reference returned does not refer to one of the parameters, it must refer to a value created within this function. However, this would be a dangling reference because the value will go out of scope at the end of the function. Consider this attempted implementation of the longest function that won’t compile:

Filename: src/main.rs

This code does not compile!
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
} even though we’ve specified a lifetime parameter 'a for the return type, this implementation will fail to compile because the return value lifetime is not related to the lifetime of the parameters at all

bcz "result" goes ut of scope and gets cleaned 



*/
