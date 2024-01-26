#![allow(dead_code)]
pub fn vectors() {
    // println!("Hello vectors");

    // let v = Vec::new(); vectors can be created this way too

    // let mut v: Vec<i32> = vec![1, 2, 3];

    // v.push(9);
    // println!("{:#?}", v);
    // [
    //     1,
    //     2,
    //     3,
    //     9,
    // ]

    let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    // we can access elements using "index"
    // let third: &i32 = &vec[2];  program to panic because it references a nonexistent element.
    // This method is best used when you want your program to crash if there’s an attempt to access an element past the end of the vector.

    // vectors are indexed by number, starting at zero.
    // access vector elements using "get" method
    // this is the safest way to get indexed element
    // if we get index out of bound then it is taken care of
    match vec.get(2) {
        // When we use the "get" method with the index passed as an argument, we get an "Option<&T>" that we can use with "match".When the "get" method is passed an index that is "outside the vector", it returns "None" without "panicking"
        Some(ele) => println!("Third element is {ele}"),
        None => println!("There is no third element"),
    }

    // let vr: Vec<&str> = vec!["a","b"];
    // println!("{:#?}",vr);

    let we = vec![1, 2, 3, 4];

    // let _first = &mut we[1];
    // we.push(6);
    // println!("changed we : {:#?} ",we);

    for i in &we {
        //Iterating over the Values in a Vector

        println!("{i}");
    }
    let mut w = vec![10, 20, 30, 40];
    for i in &mut w {
        // iterate over mutable references to change values
        *i += 50; // "*" is drefrence operator
        println!("{i}");
        // we have to use the * dereference operator to get to the value in i before we can use the += operato
        // Iterating over a vector, whether immutably or mutably, is safe because of the borrow checker's rules. If we attempted to insert or remove items in the for loop bodies in Listing 8-7 and Listing 8-8, we would get a compiler error similar to the one we got with the code in Listing 8-6. The reference to the vector that the for loop holds prevents simultaneous modification of the whole vector.
    }
    #[derive(Debug)]
    enum Spreed {
        Int(i32),
        Float(f64),
        Text(String),
    }
    // Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element
    // Using an "enum" plus a "match" expression means that Rust will ensure at compile time that every possible case is handled

    let row = vec![
        // Vectors can only store values that are the same type
        // but using enum we can store different types of data as variantd of enum come under same type (enum type)
        Spreed::Int(14),
        Spreed::Float(14.02),
        Spreed::Text(String::from("Gunjan")),
    ];
    println!("Enum Vector = {:#?}", row);
    println!("Enum Vector third position = {:#?}", row[2]); // this will access Text => Gunjan in this case

    match &row[2] {
        Spreed::Text(st) => println!("match enum {}", st),
        _ => println!("Not match"),
    }
    match &row.get(20) {
        //this will handle array out of bound
        Some(Spreed::Text(st)) => println!("match enum {}", st),
        _ => println!("Not match"),
    }
}
// Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it

// { Dropping a Vector Drops Its Elements
//     let v = vec![1, 2, 3, 4];

//     // do stuff with v
// } // <- v goes out of scope and is freed here

// When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up. The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.
