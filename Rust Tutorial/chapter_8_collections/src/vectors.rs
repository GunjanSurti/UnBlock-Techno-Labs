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
        // When we use the "get" method with the index passed as an argument, we get an "Option<&T>" that we can use with "match".
        Some(ele) => println!("Third element is {ele}"),
        None => println!("There is no third element"),
    }

    // let vr: Vec<&str> = vec!["a","b"];
    // println!("{:#?}",vr);
}
// Rust conveniently provides the vec! macro, which will create a new vector that holds the values you give it
