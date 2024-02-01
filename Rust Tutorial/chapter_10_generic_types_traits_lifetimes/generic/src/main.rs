mod generic_method;

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    println!("Hello, Generic!");

    // let char_list = vec!['q', 'e', 'f', 'a'];
    // let largest = get_largest(char_list);
    // println!("Largest char = {largest}");
    // let wont_work = Point { x: 5, y: 4.0 };
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.a(p2);

    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
generic_method::method();

}
/*
fn get_largest<T>(list: T) -> &T {
    // <T> => can be of any type
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
*/

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // Here, the generic parameters "X1" and "Y1" are declared after "impl" because they go with the "struct definition". The generic parameters "X2" and "Y2" are declared after "fn mixup", because they’re "only relevant to the method".
    fn a<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// struct Point<T> {
//     // x and y will be of same type, whatever it is
//     x: T,
//     y: T,
// }
// // can allow more generic types (eg.. T,U,V...)
// struct Point<T, U> {
//     // x and y can be same or different type
//     x: T,
//     y: U,
// }

//********************in enum************************/
// enum Option<T> {
//     Some(T),
//     None,
// }
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

/*
Generic type => to reduce code duplication

traits => constrain a generic type to accept only those types that have a particular behavior

Lifetimes => allow us to give the compiler enough information about borrowed values so that it can ensure references will be valid in more situations than it could without our help.

To define the generic "largest function", place type name declarations inside angle brackets, <>, between the name of the function and the parameter list, like this:


fn largest<T>(list: &[T]) -> &T {

*/
