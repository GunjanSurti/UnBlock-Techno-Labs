use std::fs::File;
use std::io::{self, Read};
pub fn error_2() {
    println!("error_2");
    /* Shortcuts for Panic on Error: unwrap and expect */
    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("File will not open");
    // expect will give custom error message
}
/*************************************Propagating Errors*****************************/
pub fn propagote_error() -> Result<String, io::Error> {
    let name = File::open("hello.txt");
    let mut file = match name {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
    // function is returning a value of the type Result<T, E> where the generic parameter T has been filled in with the concrete type String, and the generic type E has been filled in with the concrete type io::Error.
}
/************************shortcut "?" operator ********************************/
fn shortcut_propogate() -> Result<String, io::Error> {
    // let mut file = File::open("hello.txt")?;
    let mut uname = String::new();
    // file.read_to_string(&mut uname)?;
    // Ok(uname)

    /* The ? placed after a Result value is defined to work in almost the same way as the match expressions we defined to handle the Result values
    The "?" placed after a "Result" value is defined to work in almost the same way as the "match" expressions.
    If the value of the Result is an "Ok", the value inside the "Ok will get returned" from this expression, and the program will continue.
    "Err", the "Err" will be returned" from the whole function as if we had used the return keyword so the "error value gets propagated to the calling code".
    */
    File::open("hello.txt")?.read_to_string(&mut uname)?;
    // use the `?` operator to extract the `File` value, propagating a `Result::Err` value to the caller: `?`
    Ok(uname)
}
// The "unwrap" method is a shortcut method implemented just like the match expression. If the "Result" value is the "Ok" variant, unwrap will return the value inside the "Ok". If the Result is the "Err" variant, unwrap will call the "panic!" macro for us.

// Similarly, the expect method lets us also choose the panic! error message. Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier

// When a function’s implementation calls something that might fail, instead of handling the error within the function itself, you can "return the error to the calling code" so that it can decide what to do. This is known as propagating the error and gives more control to the calling code, where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code.

/*************************Where The ? Operator Can Be Used*****************************/

/*
The "?" operator can only be used in functions whose return type is compatible with the value the "?" is used onThis is because the ? operator is defined to perform an early return of a value out of the function, in the same manner as the match expression we defined in Listing 9-6. In Listing 9-6, the match was using a Result value, and the early return arm returned an Err(e) value. The return type of the function has to be a Result so that it’s compatible with this return.

we’re only allowed to use the "?" operator in a function that returns "Result", "Option", or another type that implements "FromResidual".

 you can use the ? operator on a Result in a function that returns Result, and you can use the ? operator on an Option in a function that returns Option, but you can’t mix and match. The ? operator won’t automatically convert a Result to an Option or vice versa in those cases, you can use methods like the ok method on Result or the ok_or method on Option to do the conversion explicitly.
*/

/* 
So far, all the main functions we’ve used return (). The main function is special because it’s the entry and exit point of executable programs, and there are "restrictions on what its return type can be for the programs to behave as expected."

Luckily, main can also return a Result<(), E>. Listing 9-12 has the code from Listing 9-10 but we’ve changed the return type of main to be Result<(), Box<dyn Error>> and added a return value Ok(()) to the end. This code will now compile:


use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> { The Box<dyn Error> type is a trait objec
     you can read Box<dyn Error> to mean “any kind of error
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
 Using ? on a Result value in a main function with the error type Box<dyn Error> is allowed, because it allows any Err value to be returned early. Even though the body of this main function will only ever return errors of type std::io::Error, by specifying Box<dyn Error>, this signature will continue to be correct even if more code that returns other errors is added to the body of main.
*/
