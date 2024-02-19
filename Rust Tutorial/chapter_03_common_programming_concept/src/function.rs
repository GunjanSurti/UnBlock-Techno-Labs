pub fn function() {
    another_one(5);
    print_labeled_measurement(5, 'g');
    let j = rt();
    println!("function with return {j}");
    let u = plus_one(99);
    println!("plus_one function {u}");
}
// Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.
fn another_one(m: i32) {
    println!("anotherOne {m}");
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    // no ";" , expression whose value we want to return.
    x + 1
}
fn rt() -> i32 {
    5 // no ";" , expression whose value we want to return.
      // There are no function calls, macros, or even let statements in the five function—just the number 5 by itself. That’s a perfectly valid function in Rust. Note that the function’s return type is specified too, as -> i32.
}
// We can define functions to have parameters, which are special variables that are part of a function’s signature. When a function has parameters, you can provide it with concrete values for those parameters. Technically, the concrete values are called arguments, but in casual conversation, people tend to use the words parameter and argument interchangeably for either the variables in a function’s definition or the concrete values passed in when you call a function.

// In function signatures, "you must declare the type of each parameter". This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean. The compiler is also able to give more helpful error messages if it knows what types the function expects.

/***************************************************************************************/

// Statements and Expressions

// Function bodies are made up of a series of statements optionally ending in an expression. So far, the functions we’ve covered haven’t included an ending expression, but you have seen an expression as part of a statement. Because Rust is an expression-based language, this is an important distinction to understand. Other languages don’t have the same distinctions, so let’s look at what statements and expressions are and how their differences affect the bodies of functions.

// "Statements" are instructions that perform some action and do not return a value.
// "Expressions" evaluate to a resultant value. Let’s look at some examples.
// Creating a variable and assigning a value to it with the let keyword is a statement.
// let y = 6;

// Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust. Consider a math operation, such as 5 + 6, which is an expression that evaluates to the value 11. Expressions can be part of statements: in Listing 3-1, the 6 in the statement let y = 6; is an expression that evaluates to the value 6. Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression, for example:

// fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {y}");
// }

// This expression:

// {
//     let x = 3;
//     x + 1 no ";" , if there is a ";" then it becomes statement and cannot be assogned to y
// }

// is a block that, in this case, evaluates to 4. That value gets bound to y as part of the let statement. Note that the x + 1 line doesn’t have a semicolon at the end, which is unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.

/********************************************************************/

//Functions with Return Values
