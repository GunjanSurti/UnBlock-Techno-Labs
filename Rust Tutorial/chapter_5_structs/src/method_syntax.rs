struct Rectangle {
    width: u32,
    height: u32,
}
// https://doc.rust-lang.org/book/ch05-03-method-syntax.html
// making "method" of struct Rectangle
impl Rectangle {
    fn _area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // we can choose to give a method the same name as one of the struct’s fields
    fn _width(&self) -> bool {
        self.width > 0
    }
}

#[derive(Debug)]
struct Square {
    _width: u32,
    _height: u32,
}
impl Square {
    fn square(size: u32) -> Self {
        Self {
            _width: size,
            _height: size,
        }
    }
}
pub fn syntax() {
    // Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else. Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and their first parameter is always self, which represents the instance of the struct the method is being called on.
    let rect1 = Rectangle {
        width: 20,
        height: 30,
    };
    // println!(
    //     " The area of the rectangle is {} square pixels.",
    //     rect1.area()
    // );
    let ret = Rectangle {
        width: 41,
        height: 74,
    };
    // as ret is Rectangle type and we have made method. every Rectangle tyoe will have "function" as "methods" specified in them
    // so thats why we can call ret.width()
    // if ret.width() {
    //     println!("The rectangle has a nonzero width; it is {}", ret.width);
    // }
    let rect2 = Rectangle {
        width: 11,
        height: 29,
    };
    //  rect1.can_hold(&ret) , for can_hold method rect1 is first and ret is second parameter
    println!("can rect1 hold ret? {}", rect1.can_hold(&ret)); // false
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2)); // true

    /*******************************************************************************************************/
    // Associated Functions

    // All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl. We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with. We’ve already used one function like this: the String::from function that’s defined on the String type.
    let sq = Square::square(8);
    // the "::" syntax is used for both associated functions and namespaces created by modules
    println!("{:#?}", sq);

    // Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct. These are often called "new", but new isn’t a special name and isn’t built into the language. For example, we could choose to provide an associated function named square that would have one dimension parameter and use that as both width and height, thus making it easier to create a square Rectangle rather than having to specify the same value twice:
}

// the following are the same:

// p1.distance(&p2);
// (&p1).distance(&p2);

// Multiple impl Blocks
// Each struct is allowed to have multiple impl blocks. For example, Listing 5-15 is equivalent to the code shown in Listing 5-16, which has each method in its own impl block.

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
// Listing 5-16: Rewriting Listing 5-15 using multiple impl blocks

// There’s no reason to separate these methods into multiple impl blocks here, but this is valid syntax. We’ll see a case in which multiple impl blocks are useful in Chapter 10, where we discuss generic types and traits.
