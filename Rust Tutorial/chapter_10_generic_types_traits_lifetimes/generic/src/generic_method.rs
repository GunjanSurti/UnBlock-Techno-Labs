pub fn method() {
    // println!("g method");
    let pt: Point<f32> = Point { x: 5.0, y: 1.0 };
    let q = pt.distance_from_origin();
    println!("{q}");

    let w = pt.x();
    println!("{w}");

    let pt2 = Point { x: 7, y: 6 };
    
    println!("{:#?}",pt2.x());
}
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        println!("w",);
        &self.x
    }
}

impl Point<f32> {
    // here, Point<f32> will have a distance_from_origin method;
    //  other instances of Point<T> where T is not of type f32 will not have this method define
    // if "T" is of type f32 it will have "distance_from_origin()" and "x()" methods available 
    // other have only "x()" method available 
    // means we are giving a type a specific method to use 
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Rust accomplishes this by performing ""monomorphization"" of the code using generics at compile time. "Monomorphization" is the process of "turning generic code into specific code" by filling in the concrete types that are used when compiled. In this process, the compiler does the opposite of the steps we used to create the generic function the compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.

// "PartialOrd" : Some items of this type can be compared and ordered. "Ord" : All items of this type can be compared and ordered.