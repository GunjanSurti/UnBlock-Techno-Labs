mod refrences;
fn main() {
    let _fix_str = "hello";
    // this is in stack and cannot be changed
    let st = String::from("Hello");
    // dynamic in size , heap allocation can be appended to diff string
    // let st2 = st; // move not copy, means st is deleted by compiler
    let st2 = st.clone(); // now st will remain in memory

    let c = 6;
    let d = c;
    // copy (integers, boolean, characters) will copy not move
    /******************************************************************** */
    println!(" original c: {c}, copy d: {d}, original st: {st}, clone st2: {st2} ");
    let s = String::from("hell"); // fixed sixe
    takes_ownership(s.clone());
    // if only s is passed then below line will not print as s is moved
    println!("Fixed size string using &str : {s}");
    let go = gives_ownership();
    println!("Giving owner ship to variable go: {go}"); // go will become "Gave OwnerShip" string
    refrences::refrences();
}
fn takes_ownership(some_string: String) {
    // some_string will become s (parameter that is passed) as it is string
    // if it was integer then it is copied
    println!("Takes Ownership : {some_string}");
}
fn gives_ownership() -> String {
    let some_string = String::from("Gave OwnerShip");
    some_string // no ";" (expression and we want to return)

    // we can also give and take back ownership
}

// stack also store stack frames (each function) and stack frames store local variables

// let mut s = String::from("hello");

//     s.push_str(", world!"); // push_str() appends a literal to a String

//     println!("{}", s); // This will print `hello, world!`

// String be mutated but literals cannot

// Literals are constant values assigned to constant variables. Literals represent fixed values that cannot be modified. Literals are a synthetic representation of boolean, character, numeric, or string data, a medium of expressing particular values in the program
// the memory is automatically returned once the variable that owns it goes out of scope

// let s = "hello";
// The variable s refers to a string literal, where the value of the string is hardcoded into the text of our program. The variable is valid from the point at which it’s declared until the end of the current scope. Listing 4-1 shows a program with comments annotating where the variable s would be valid.

//     {                      // s is not valid here, it’s not yet declared
//         let s = "hello";   // s is valid from this point forward

//         // do stuff with s
//     }                      // this scope is now over, and s is no longer valid
