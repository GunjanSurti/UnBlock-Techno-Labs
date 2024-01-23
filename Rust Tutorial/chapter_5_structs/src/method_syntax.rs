struct Rectangle {
    width: u32,
    height: u32,
}
// making "method" of struct Rectangle
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
pub fn syntax() {
    // Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else. Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and their first parameter is always self, which represents the instance of the struct the method is being called on.
    let rect1 = Rectangle {
        width: 20,
        height: 30,
    };
    println!(" The area of the rectangle is {} square pixels.",rect1.area());
}
