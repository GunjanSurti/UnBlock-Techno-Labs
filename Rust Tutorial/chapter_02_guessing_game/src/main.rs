extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the Number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // random number generate

    // println!("The secret number is: {secret_number}");
    loop {
        println!("Please input your guess");

        // we made mutable variable 
        let mut guess = String::new(); // this will take user input, and it is changable

        // this will take input from user
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //this will remove intendation and remove "\r\n" and convert string to number 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // continue in the loop
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            // the cmp method will return Ordering::... as per condition and
            Ordering::Less => println!("Too Small"), // this is arm which will be checked by match one-by-one
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}

// cargo doc --open will generate documentation

//   When the code compares 50 to 38, the ""cmp" method will return Ordering::Greater" because 50 is greater than 38. The match expression gets the Ordering::Greater value and starts checking each arm’s pattern. It looks at the first arm’s pattern, Ordering::Less, and sees that the value Ordering::Greater does not match Ordering::Less, so it ignores the code in that arm and moves to the next arm. The next arm’s pattern is Ordering::Greater, which does match Ordering::Greater! The associated code in that arm will execute and print Too big! to the screen. The match expression ends after the first successful match, so it won’t look at the last arm in this scenario.
