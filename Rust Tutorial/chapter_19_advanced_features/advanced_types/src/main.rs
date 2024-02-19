fn main() {
    // Creating Type Synonyms with Type Aliases
    type Kilometer = i32;
    let x = 5;
    let y: Kilometer = 9;
    // here we have given alias

    println!("x + y = {} + {} ", x, y);

    //The main use case for type synonyms is to "reduce repetition"
    type Thunk = Box<dyn Fn() + Send + 'static>;
    // "thunk" is a word for code to be evaluated at a later time, so it’s an appropriate name for a closure that gets stored
    let _f: Thunk = Box::new(|| println!("hi"));

    fn _takes_long_type(_f: Thunk) {
        // fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }

    // fn returns_long_type() -> Thunk {
    // fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
    // --snip--
    // }

    // dynamically sized type called str
    // let s1: str = "Hello there!";    
    // let s2: str = "How's it going?";
    // both above two sentence will not compile 
}

// The Never Type that Never Returns
fn bar() -> ! {
    // --snip--
    //the function bar returns never
    panic!(); // this method does not return anything
}


/*
the concept of dynamically sized types. Sometimes referred to as "DSTs" or unsized types, these types let us write code using values whose size we can know only at "runtime".

That’s right, not "&str", but "str" on its own, is a DST. We can’t know how long the string is until runtime, meaning we can’t create a variable of type str, nor can we take an argument of type str

*/