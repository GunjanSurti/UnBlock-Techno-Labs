enum Coin {
    // Penny,
    // Nickel,
    // Dime,
    Quarter(UsState),
}
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    // Alaska,
}
pub fn option() {
    let some_number: Option<i32> = Some(8);
    // let some_char: Option<char> = Some('e');
    println!("{:#?}", some_number);

    let none: Option<i32> = None; // Some(8,) , here we have to give its type
    println!("{:#?}", none); // => None

    let _co = match_coin(Coin::Quarter(UsState::Alabama));
    // println!("Coin Dime has value : {co}");
    // let eight = Some(8);
    // let a = plus_one(eight);
    // above and below are same
    let _a = plus_one(Some(8));
    // println!("a : {:#?}", a);
}
// fn good(){
//     println!("good");
// }

// fn match_all(x: i32)  {
//     match x { // rolling dice
//         6 => good(),
//         5 => good(),
//         // _ => String::from("Reroll"),
//         // "_" means for other pattern it will Reroll
//         _ => (),
//         // _ is a special pattern that matches any value and does not bind to that value.
//         // we’re telling Rust explicitly that we aren’t going to use any other value that doesn’t match a pattern in an earlier arm, and we don’t want to run any code in this case.
//     }
// }
fn match_coin(coin: Coin) -> u8 {
    match coin {
        // Coin::Penny => {
        //     // using {} for multiple line
        //     8
        // } // no comma need
        // Coin::Nickel => 5,
        // Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:#?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // here we are matching arms of match and every pattern must be covered
        None => None,
        Some(i) => Some(i + 1),
    }
}
// this will show an error
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }
// The Option<T> enum is so useful that it’s even included in the prelude; you don’t need to bring it into scope explicitly. Its variants are also included in the prelude: you can use Some and None directly without the Option:: prefix. The Option<T> enum is still just a regular enum, and Some(T) and None are still variants of type Option<T>.

// The <T> syntax is a feature of Rust we haven’t talked about yet. It’s a generic type parameter, and we’ll cover generics in more detail in Chapter 10. For now, all you need to know is that <T> means that the Some variant of the Option enum can hold one piece of data of any type, and that each concrete type that gets used in place of T makes the overall Option<T> type a different type

// cannot add as they are of different type
// let x: i8 = 5;
// let y: Option<i8> = Some(5);

// let sum = x + y;

// you have to convert an Option<T> to a T before you can perform T operations with it. Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.

// This seems very similar to a conditional expression used with if, but there’s a big difference: with if, the condition needs to evaluate to a Boolean value, but here it can be any type. The type of coin in this example is the Coin enum that we defined on the first line.

// Next are the "match" arms. An arm has two parts: a "pattern" and "some code". The first arm here has a pattern that is the value "Coin::Penny" and then the "=>" operator that separates the pattern and the code to run. The code in this case is just the value 1. Each arm is separated from the next with a comma.

// We can have as many arms as we need.The code associated with each arm is an expression, and the resultant value of the expression in the matching arm is the value that gets returned for the entire match expression.
