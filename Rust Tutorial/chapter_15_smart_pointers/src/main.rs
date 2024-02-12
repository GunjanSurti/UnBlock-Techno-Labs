#[derive(Debug)]
#[allow(dead_code)]
enum List {
    // Cons List
    Cons(i32, Box<List>), // recursive type
    Nil,
}

#[allow(unused_imports)]
use List::{Cons, Nil};
mod deref;
mod drop;
mod refrence_counting;
fn main() {
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    //  box’s pointer data

    // println!("{:#?}", list);
    // deref::deref();
    // drop::drop();

    refrence_counting::refrece_counting();
}

/*
-------------------------------Using a Box<T> to Store Data on the Heap-----------------------------

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
Just like any owned value, when a box goes out of scope, as "b" does at the end of main, it will be deallocated. The deallocation happens both for the box (stored on the stack) and the data it points to (stored on the heap).

A cons(construct function) list is a data structure

Vec<T> is better than Cons List

Conceptually, we still have a list, created with lists holding other lists, but this implementation is now more like placing the items next to one another rather than inside one another.
*/
