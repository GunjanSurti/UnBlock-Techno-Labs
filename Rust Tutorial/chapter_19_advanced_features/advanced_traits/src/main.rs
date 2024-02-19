pub trait Iterator {
    type Item;
    // this is "associated type"
    // this is unknown and we define when we implement it
    // The type Item is a "placeholder"
    fn next(&mut self) -> Option<Self::Item>;

    // Implementors of the Iterator trait will specify the concrete type for Item, and the next method will return an Option containing a value of that concrete type.
}
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    // Default Generic Type Parameters and Operator Overloading
    // here we are overriding Add method so it can add two Points struct
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

trait Pilot {
    fn fly(&self);
}

/*
Difference b/w associated type and generice is that associated type has one concrete type per implementaions
and generice type has multiple concrete type per implementations
*/
