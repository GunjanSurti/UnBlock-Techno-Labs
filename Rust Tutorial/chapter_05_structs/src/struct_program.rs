#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub fn prgm() {
    let scale = 2;
    let rect = Rectangle {
        // width: dbg!(30 * scale), // => [src\struct_program.rs:10] 30 * scale = 60
        // used to know what is our code doing
        width: 30 * scale, // => [src\struct_program.rs:10] 30 * scale = 60
        height: 50,
    };
    // println!() takes references
    let _rc = area(&rect);
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rect)
    // );
    // dbg takes ownership and gives value
    // The dbg! macro can be really helpful when you’re trying to figure out what your code is doing!
    // dbg!(&rect); // "&" bcz we dont want "dbg" to take ownership
}

fn area(rectangle: &Rectangle) -> u32 {
    //&Rectangle borrowing struct
    rectangle.width * rectangle.height
}
