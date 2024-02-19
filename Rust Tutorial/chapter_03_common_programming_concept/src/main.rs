mod control_flow;
mod control_flow_2;
mod function;
mod main_2;

fn main() {
    let x = 5;

    let x = x + 1;
    // x = 6 this is called shadowing
    // We can shadow a variable by using the same variable’s name and repeating the use of the let keyword

    {
        // this is called inner scope
        let x = x * 2; // x = 6 * 2 = 12
        println!("The value of x in the inner scope is: {x}");
    } // as the scope ends ... x = 6

    println!("The value of x is: {x}");
    // x = 6
    // const Y: u8 = 7; // for const type is neccessary and value should be assigned
    // by using "let" we can change "Variable's type", and can reassign reusing "let"
    // by using "mut" we can change value
    let mut m = 8;
    println!("old m = {m}");
    m = 9;
    println!("after changing m = {m}");
    let s = "      "; // 6 blank spaces
    let s = s.len();
    println!("after changing variable type = {s}");

    // Data types
    // compound -> tuple
    let tup = (500, 2.9, 1); // this is tuple , cannot change length,can be diff val
    let (a, b, c) = tup; // we pulled values from tup, called destructuring
    println!("value of a = {a}, b = {b}, c = {c}");

    //access a tuple element directly by using a period (.) followed by the index of the value we want to access.
    let q = (466, 8.5);
    let e = q.0;
    let w = q.1;
    println!("e = {e}, w = {w}");
    // let r = (); this is called "unit" an empty tuple
    /*******************************************************************************************************/
    // array fixed
    // You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:

    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    // let a = [3; 5]; => [3, 3, 3, 3, 3]; {[initial val, no. of times]}
    // An array is a single chunk of memory of a known, fixed size that can be allocated on the "stack". You can access elements of an array using indexing,
    let _first = _a[0];
    let _second = _a[1];

    main_2::main_2();
    function::function();
    control_flow::flow();
    control_flow_2::flow_2();
}

/*******************************************************************************************************/

// Compile-time constants, compile-time evaluable functions, and raw pointers.

// Compile-time constants
// Sometimes a certain value is used many times throughout a program, and it can become inconvenient to copy it over and over. What's more, it's not always possible or desirable to make it a variable that gets carried around to each function that needs it. In these cases, the const keyword provides a convenient alternative to code duplication:

// const THING: u32 = 0xABAD1DEA;

// let foo = 123 + THING;
// Constants must be explicitly typed; unlike with let, you can't ignore their type and let the compiler figure it out. Any constant value can be defined in a const, which in practice happens to be most things that would be reasonable to have in a constant (barring const fns). For example, you can't have a [File] as a const.

// The only lifetime allowed in a constant is 'static, which is the lifetime that encompasses all others in a Rust program. For example, if you wanted to define a constant string, it would look like this:

// const WORDS: &'static str = "hello rust!";
// Thanks to static lifetime elision, you usually don't have to explicitly use 'static:

// const WORDS: &str = "hello convenience!";
// const items looks remarkably similar to static items, which introduces some confusion as to which one should be used at which times. To put it simply, constants are inlined wherever they're used, making using them identical to simply replacing the name of the const with its value. Static variables, on the other hand, point to a single location in memory, which all accesses share. This means that, unlike with constants, they can't have destructors, and act as a single value across the entire codebase.

// Constants, like statics, should always be in SCREAMING_SNAKE_CASE.

// For more detail on const, see the Rust Book or the Reference.

// Compile-time evaluable functions
// The other main use of the const keyword is in const fn. This marks a function as being callable in the body of a const or static item and in array initializers (commonly called "const contexts"). const fn are restricted in the set of operations they can perform, to ensure that they can be evaluated at compile-time. See the Reference for more detail.

// Turning a fn into a const fn has no effect on run-time uses of that function.

// Other uses of const
// The const keyword is also used in raw pointers in combination with mut, as seen in *const T and *mut T. More about const as used in raw pointers can be read at the Rust docs for the [pointer primitive].

// The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime
// Rust’s naming convention for constants is to use all uppercase with underscores between words.

//  The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
//     let spaces = "   ";
//     let spaces = spaces.len();
// The first spaces variable is a string type and the second spaces variable is a number type. Shadowing thus spares us from having to come up with different names, such as spaces_str and spaces_num; instead, we can reuse the simpler spaces name. However, if we try to use mut for this, as shown here, we’ll get a compile-time error:
// Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time.
//*************************************************************************************************/
// Data Types
// 1. Scalar Types 2. Compound

//1. Scalar:-  A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. You may recognize these from other programming languages. Let’s jump into how they work in Rust.

// 2. Compound Types
// Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
// 2.1 The Tuple Type
// A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
// 2.2 The Array Type
// Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.
/*******************************************************************************************************/
// String slices.

// See also the std::str module.

// The str type, also called a 'string slice', is the most primitive string type. It is usually seen in its borrowed form, &str. It is also the type of string literals, &'static str.

// Basic Usage
// String literals are string slices:

// let hello_world = "Hello, World!";
// Here we have declared a string slice initialized with a string literal. String literals have a static lifetime, which means the string hello_world is guaranteed to be valid for the duration of the entire program. We can explicitly specify hello_world's lifetime as well:

// let hello_world: &'static str = "Hello, world!";
// Representation
// A &str is made up of two components: a pointer to some bytes, and a length. You can look at these with the [as_ptr] and [len] methods:

// use std::slice;
// use std::str;

// let story = "Once upon a time...";

// let ptr = story.as_ptr();
// let len = story.len();

// // story has nineteen bytes
// assert_eq!(19, len);

// // We can re-build a str out of ptr and len. This is all unsafe because
// // we are responsible for making sure the two components are valid:
// let s = unsafe {
//     // First, we build a &[u8]...
//     let slice = slice::from_raw_parts(ptr, len);

//     // ... and then convert that slice into a string slice
//     str::from_utf8(slice)
// };

// assert_eq!(s, Ok(story));
// Note: This example shows the internals of &str. unsafe should not be used to get a string slice under normal circumstances. Use as_str instead.

// Invariant
// Rust libraries may assume that string slices are always valid UTF-8.

// Constructing a non-UTF-8 string slice is not immediate undefined behavior, but any function called on a string slice may assume that it is valid UTF-8, which means that a non-UTF-8 string slice can lead to undefined behavior down the road.

/*******************************************************************************************************/

// fn main() {
// use of let again and again
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }
// If we have a type that does not have a definite size, we call those primitive types 'usize' in rust or dynamically sized types in rust
