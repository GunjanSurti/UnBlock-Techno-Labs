use std::ops::Deref;
struct MyBox<T>(T);
// this is tuple struct which strores 1 generic value

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    // Associated type

    // method name should not change 
    // fn deref(&self) -> &Self::Target { OR
    fn deref(&self) -> &T {
        // here we are returning refrence not the actual value
        // bcz if return value then ownership will be moved outside
        &self.0
        // as MyBox is tuple struct we are returning first element
        // we want to return the refrence stored in tuple struct.
        // with the "deref trait" allows the compiler to take any value that implenments "Deref"
        // call the deref method to get a refrencewhich the compiler knows how to deref.
    }
}

#[allow(dead_code)]
pub fn deref() {
    let x = 5;
    assert_eq!(5, x);

    let y = &x; // y => memory addressor pointer, it points to memory location where 5 is stored
    assert_eq!(5, *y);
    // *y => derefrencing y means we are following memory address (stored in y) to the actual value stroed in memory

    let z = Box::new(x);
    // just as refrence "Box" is pointing a value stored somewhere in memory
    // only difference is that z is pointing to "copy" of 5 (as integer is primitive type )
    assert_eq!(5, *z); // box also has deref trait

    let a = MyBox::new(x);
    assert_eq!(5, *a); // first we cannot derefrence as there is no trait, we have to add that trait

    // assert_eq!(5, *a); => rust actually calls (in this case) assert_eq!(5, *(a.deref())); (this will run too)
    // so first we call deref() to get refrence and then "*"(derefrence) operator
    // rust calls automatically

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // &MyBox<String> ->&String -> &str
    // MyBox and String both implement Deref trait so refrece will convert from one to another(automatically)
}

#[allow(dead_code)]
fn hello(name: &str) {
    // Implicit Deref Coercions with Functions and Methods
    // even though function takes &str refrence type we can send MyBox type refrence
    println!("Hello!, {}", name);
}
// ----------------------------Implicit Deref Coercions with Functions and Methods------------------------------
// Deref trait will convert a refrence to one type to refrence to another type
/*
 * without "deref trait" compiler only knows how to derefrence refrences
 * Deref => override deref trait for immutable refrence
 * DerefMut => override deref trait for mutable refrences
 * Rust cannot peform deref coercion when goig from immutable refrence to mutable refrence
 * converting mutable to immutable will not break the rule
 * Rust does deref coercion when it finds types and trait implementations in three cases:

From &T to &U when T: Deref<Target=U>
From &mut T to &mut U when T: DerefMut<Target=U>
From &mut T to &U when T: Deref<Target=U>
*/
