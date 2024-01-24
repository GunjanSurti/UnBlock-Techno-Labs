pub fn if_let() {
    let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }
    // shorter way
    if let Some(max) = config_max {
        // here Some(max) is the pattern
        println!("The maximum is configured to be {}", max);
    } else {
    }
}

// The "if let" syntax lets you combine if and let into a less verbose way to handle values that match one pattern while "ignoring the rest"
// you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
// We can include an else with an if let. The block of code that goes with the else is the same as the block of code that would go with the _ case in the match expression that is equivalent to the if let and else
