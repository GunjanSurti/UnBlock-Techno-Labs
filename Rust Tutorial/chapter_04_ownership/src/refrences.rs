pub fn refrences() {
    // the rules of refrences
    // 1. at any given time , you can have either one mutable refrence or any number of immutable refrences
    //
    //2. refrences must always be valid

    let s = String::from("Hello World");

    let a = &s[..4]; // =>  first 4 characters also can be [0..4]
    let b = &s[..]; // => whole string
    let c = &s[6..]; // from 6 index upto end (range)
    println!("a:{a}, b:{b}, c:{c} ");

    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("length of s1 : '{s1}' is {len}");

    let mut s2 = String::from("Hello");
   let asd =  change_refrence(&mut s2);
   println!("{asd}");
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
    // Here, s goes out of scope. But because it does not have ownership of what
    // it refers to, it is not dropped.
    //   Just as variables are immutable by default, so are references. We’re not allowed to modify something we have a reference to.
}

fn change_refrence(s: &mut String) -> &String {
    s.push_str(",World!");
    println!("{s}");
    s
}

// let mut s = String::from("hello");

// let r1 = &s; // no problem
// let r2 = &s; // no problem 
// if we write mutable after this line it will cause error as mutable and immutable will be same 
// println!("{} and {}", r1, r2);
// // variables r1 and r2 will not be used after this point

// let r3 = &mut s; // no problem
// println!("{}", r3);

// Dangling References
// In languages with pointers, it’s easy to erroneously create a dangling pointer—a pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

// Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. But we tried to return a reference to it. That means this reference would be pointing to an invalid String. That’s no good! Rust won’t let us do this.

// The solution here is to return the String directly:


// fn no_dangle() -> String {
//     let s     = String::from("hello");

//     s
// }