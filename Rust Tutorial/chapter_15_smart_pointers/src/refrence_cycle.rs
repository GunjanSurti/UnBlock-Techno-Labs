use crate::refrence_cycle::List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub fn ref_cycles() {
    use super::*;
    // let a: Rc<List> = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
}
