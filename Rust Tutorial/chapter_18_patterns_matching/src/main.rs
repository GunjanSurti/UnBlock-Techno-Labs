fn main() {}

/*
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    a is at index 0
b is at index 1
c is at index 2

* enumerate() => The iterator returned yields pairs (i, val), where i is the current index of iteration and val is the value returned by the iterator.
 *
 *
*/

/* destructuring in function para

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
*/

/*

Patterns come in two forms: "refutable" and "irrefutable".
 *
 *  Patterns that will match for any possible value passed are "irrefutable"
 *  Patterns that can fail to match for some possible value are "refutable".

*/

/*

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
*/

/* multiple Patterns using(|)
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
*/

/* Matching Ranges of Values with (..=)

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
*/
/*struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    // or let Point { x, y } = p; // taking x,y out of p
    let Point { x: a, y: b } = p;
    // here we are taking x,y from p and assigning a,b
    assert_eq!(0, a);
    assert_eq!(7, b);
}

*/

/*
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
 */

/*  Destructuring Nested Structs and Enums


 enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }
}
 */
/*Destructuring Structs and Tuples

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

*/

/* Ignoring an Entire Value with _

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
*/

/* Ignoring Parts of a Value with a Nested _

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        // here we don't care about actual value but we want to see if there is an value or not
        // if there is then dont change the value
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);


    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }


*/

/* Ignoring Remaining Parts of a Value with (..)
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }


    fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}

fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => { **************panics********************
            // as compiler dont know which value to assign
            println!("Some numbers: {}", second)
        },
    }
}

*/

/*    Extra Conditionals with "Match Guards"

 let num = Some(4);

    match num {
        // if first arm gives 0 after x % 2
        //A match guard is an additional if condition, specified after the pattern in a match arm, that must also match for that arm to be chosen
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

*/

/*

let x = 4;
let y = false;

match x {
    // The match condition states that the arm only matches if the value of x is equal to 4, 5, or 6 and if y is true
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),

    / here behaves it is (4 | 5 | 6) if y => ... not   4 | 5 | (6 if y) => ...

}
*/

/* @ Bindings

enum Message {
        Hello { id: i32 },
    }
let msg = Message::Hello { id: 5 };

match msg {
    // The at operator @ lets us create a variable that holds a value at the same time as we’re testing that value for a pattern match
    Message::Hello{
        //The id field’s value could have been 10, 11, or 12
        id: id_variable @ 3..=7 // here we are assigning "id" to "id_variable"
    } => println!("Found an id in range: {}", id_variable),

    Message::Hello { id: 10..=12 } => {
        // here we are not binding to any variable, so any value will match here
        println!("Found an id in another range")
    }
}
*/
