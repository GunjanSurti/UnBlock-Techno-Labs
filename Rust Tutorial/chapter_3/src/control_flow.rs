pub fn flow() {
    println!("hello flow");
    let ab: i32 = 6;
    if ab > 9 {
        // condition must be Booleans
        println!("less ");
        //  Blocks of code associated with the conditions in if expressions are sometimes called arm
    } else {
        println!("more");
    }
    let number = 77;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2, the number is {number}");
    }

    // let num = if number == 2 { 5 } else { "six" };  this will not work as diff type of expression in both
    let num = if number == 2 { 5 } else { 9 }; // no ";" in {} condition
                                               // the values from each arm is same and it should be
                                               // Remember that blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions
    println!("{num}");
    // Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable
    /*****************************************************************************************/
    // Loops => Rust has three kinds of loops: "loop", "while", and "for".
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 11 {
            break counter * 2; // result = 22 , write value after "break" to return
        } // no ";" , as it is expression , which will assign value to result   
    };
    println!("The result is {result}");
}

// If you don’t provide an else expression and the condition is false, the program will just skip the if block and move on to the next bit of code.

// Using too many else if expressions can clutter your code, so if you have more than one, you might want to refactor your code. Chapter 6 describes a powerful Rust branching construct called match for these cases.

// You can place the "break" keyword within the loop to tell the program when to stop executing the loop
// We also used "continue" in the guessing game, which in a loop tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.

// One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job. You might also need to "pass the result" of that operation out of the loop to the rest of your code. To do this, you can add the "value you want returned after the break expression" you use to stop the loop; that value will be returned out of the loop so you can use it
