use std::{net::IpAddr, pin};

pub fn panic() {
    println!("hello panic");
    let home: IpAddr = "127.0.0.1".parse().expect("this should be valid IpAddress");
    // here the compiler doesnt understand that ip addresss is write or not
    println!("{}", home);
}

// returning "Result" is a good choice when defining a function that might fail
// panic => stop execution
// Result => handle the problem
// In situations such as "examples, prototype code, and tests", it’s more appropriate to write code that panics instead of returning a Result

// unwrap or expect => when we want whole test to fail
// The unwrap method is used to extract the value from an Option or Result type, panicking if the value is None or Err . The expect method is similar but allows you to specify a custom error message
// parse method => returns "Result" and converts string to numbers

/*********************************Guidelines for Error Handling *************************
It’s advisable to have your code panic when it’s possible that your code could end up in a bad state. In this context, a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:

The bad state is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.
Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
There’s not a good way to encode this information in the types you use.

If someone calls your code and passes in values that don’t make sense, it’s best to return an error if you can so the user of the library can decide what they want to do in that case. However, in cases where continuing could be insecure or harmful, the best choice might be to call panic! and alert the person using your library to the bug in their code so they can fix it during development.
Similarly, panic! is often appropriate if you’re calling external code that is out of your control and it returns an invalid state that you have no way of fixing.

when "failure is expected", it’s more appropriate to return a "Result" than to make a panic! call.

*/
