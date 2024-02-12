enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;

use List::{Cons, Nil};

pub fn refrece_counting() {
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // let b = Cons(3, Rc::clone(&a));
    // /******************Or both are same****************** */
    // let c = Cons(4, a.clone());
    // // a.clone as Cons(..,()) has Rc<List> as 2nd argument
    // // Rc::clone() dosent make deep copies of data like most clone implementations
    // // instead it just increases the refrence count

    /* to count refrence we use Rc::strong_count(<..>) */

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("Count after creating a {}", Rc::strong_count(&a));
    let _b = Rc::new(Cons(3, Rc::clone(&a)));
    println!("Count after creating b {}", Rc::strong_count(&a));
    {
        let _c = Rc::new(Cons(4, Rc::clone(&a)));
        println!("Count after creating c {}", Rc::strong_count(&a));
        // goes out of scope so "c" is droped and refrece_count decrease
    }
    println!("Count after c goes out of scope {}", Rc::strong_count(&a));
    /*
    Count after creating a 1
    Count after creating b 2
    Count after creating c 3
    Count after c goes out of scope 2
     */

    // when we go out of main the refrece_count will be 0
    // refrece_count will only allow multiple parts of code to read-only not modify
    //Rc<T> is only for use in single-threaded scenarios
    // Rc<T> allows multiple owners of data with refrence counting
}
