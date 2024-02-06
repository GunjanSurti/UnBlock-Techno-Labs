use chapter_12_command_line_program::Config;
use std::{env, process};
fn main() {
    let args: Vec<String> = env::args().collect();
    // used to take arguments from command line

    let config = Config::build(&args).unwrap_or_else(|err| {
        //unwrap_or_else allows us to define some custom, non-panic! error handling
        //If the Result is an Ok value, this method’s behavior is similar to unwrap: it returns the inner value Ok is wrapping
        eprintln!("Problem parsing arguments: {}", err);
        // err => not enough arguments
        process::exit(1);
    }); // this is to reduce noise which is shown in terminal in case of error

    println!("Searching for {:?}", config.query);
    println!("In file {:?}", config.filename);

    // here we dont use unwrap_or_else as we dont want success value, we only want error value
    if let Err(e) = chapter_12_command_line_program::run(config) {
        eprintln!("Application Error: {}", e);
        // this will not be written in new file in case of error
        // means this will show up in terminal and not in new file, here we are trying to write output in new file
        process::exit(1);
    }
}

/***
 * cargo run -- file poem.txt
 * two hyphens to indicate the following arguments are for our program rather than for cargo
 * *******Separation of Concerns for Binary Projects*******
 * =>Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
=>As long as your command line parsing logic is small, it can remain in main.rs.
=>When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

The responsibilities that remain in the main function after this process should be limited to the following:

=>Calling the command line parsing logic with the argument values
=>Setting up any other configuration
=>Calling a run function in lib.rs
=>Handling the error if run returns an error

This pattern is about separating concerns: main.rs handles running the program, and lib.rs handles all the logic of the task at hand. Because you can’t test the main function directly, this structure lets you test all of your program’s logic by moving it into functions in lib.rs.
 The code that remains in main.rs will be small enough to verify its correctness by reading it.
 *
 *There are a number of ways we could manage the "String data"; the easiest, though somewhat inefficient, route is to call the "clone" method on the values. This will make a full copy of the data for the Config instance to own, which takes more time and memory than storing a reference to the string data. However, cloning the data also makes our code very straightforward because we don’t have to manage the lifetimes of the references; in this circumstance, giving up a little performance to gain simplicity is a worthwhile trade-off.

Box<dyn Error> means the function will return a type that implements the Error trait, but we don’t have to specify what particular type the return value will be. This gives us flexibility to return error values that may be of different types in different error cases. The dyn keyword is short for “dynamic.”

***********************************writing output in different file************************
cargo run to poem.txt > output.txt  // this will create and write in this file
 the ">" means write output in this file( if not present then create it...)
 */
