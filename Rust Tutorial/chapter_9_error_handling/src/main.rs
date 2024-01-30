#![allow(dead_code)]
#![allow(unused)]
use core::panic;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
use std::pin;

use errors::err;
mod errors;
mod errors_2;
mod panic_1;
fn main() {
    let file_result = File::open("hello.txt");
    /* File::open returns Result<T,E>
       File::open might give success or fail so thats why it retrun type is Result<T,E>
       if success => instance of "Ok"
       if fail => instance of "Err"
    */
    // errors::err();
    // errors::err_match();
    // errors_2::error_2();
    panic_1::panic();
}

// it is essential to note the type of error
//  type "Result<T, E>" for "recoverable errors"
//  "panic!" macro that stops execution when the program encounters an "unrecoverable "error.

/************Unwinding the Stack or Aborting in Response to a Panic ********************/
// By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. However, this walking back and cleanup is a lot of work. Rust, therefore, allows you to choose the alternative of immediately aborting, which ends the program without cleaning up.

// Memory that the program was using will then need to be cleaned up by the operating system. If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile] sections in your Cargo.toml file. For example, if you want to abort on panic in release mode, add this:
/*
[profile.release]
panic = 'abort'
*/

// fn main() {
//     let v = vec![1, 2, 3];

//     v[99];
// } // $ RUST_BACKTRACE=1 cargo run => runs in bash

/*
enum Result<T, E> { this is already in global scope, in "prelude"
    Ok(T),
    Err(E),
}
The "T" and "E" are generic type parameters
"T" represents the type of the value that will be returned in a success case within the "Ok variant", and "E" represents the type of the error that will be returned in a failure case within the "Err variant"
Because Result has these generic type parameters, we can use the Result type and the functions defined on it in many different situations where the successful value and error value we want to return may differ.

*/
