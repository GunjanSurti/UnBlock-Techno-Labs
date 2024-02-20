fn main() {
    println!("Hello, world!");
}

/*
The term macro refers to a family of features in Rust: "declarative macros" with macro_rules! and three kinds of "procedural macros":
* Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
* Attribute-like macros that define custom attributes usable on any item
* Function-like macros that look like function calls but operate on the tokens specified as their argument

macros are a way of writing code that writes other code, which is known as metaprogramming

*/
