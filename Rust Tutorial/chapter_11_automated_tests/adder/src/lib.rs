#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}
#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/*******************************Above is our product code ******************************************/
/*******************************Below is our Test code ******************************************/
#[cfg(test)]
mod tests {
    // we have to bring into scope
    use super::*; // we are bringing in everything from parent module, we are refrencing parent module
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }
    #[test]
    fn smaller_cannot_hold_larger(){
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger)); // we have written not "!" 
    }
}
