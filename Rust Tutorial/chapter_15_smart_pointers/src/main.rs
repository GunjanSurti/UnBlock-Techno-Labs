#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:#?}", list)
}

/*

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
Just like any owned value, when a box goes out of scope, as b does at the end of main, it will be deallocated. The deallocation happens both for the box (stored on the stack) and the data it points to (stored on the heap).


*/
