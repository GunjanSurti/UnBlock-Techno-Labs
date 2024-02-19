fn main() {}

pub trait Iterator {
    type Item;
    // this is "associated type"
    // this is unknown and we define when we implement it
    // The type Item is a "placeholder"
    fn next(&mut self) -> Option<Self::Item>;

    // Implementors of the Iterator trait will specify the concrete type for Item, and the next method will return an Option containing a value of that concrete type.
}

/*
Difference b/w associated type and generice is that associated type has one concrete type per implementaions
and generice type has multiple concrete type per implementations
*/
