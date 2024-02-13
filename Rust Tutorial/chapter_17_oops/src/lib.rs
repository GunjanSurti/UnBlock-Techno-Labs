pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // we are using traits not genreics because we want to have multiple type
    // if genreics then any type but only single type is allowed
    // means our screen components can store buttons OR textField OR etc but not all together if use generics
    pub components: Vec<Box<dyn Draw>>,
    // Vec<Box<dyn Draw>> => our Box contains any type that contains Draw Trait
    // we have vectore of trait objects
    // dyn => dynamic dispatch
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub lable: String,
}

impl Draw for Button {
    fn draw(&self) {
        //draw button 
    }
}
// /*************************Encaplutation********************/
// pub struct AveragedCollection {
//     list: Vec<i32>, // instead of stroing in vector we could store in has set, our methods "add" and "remove" will change
//     average: f64,
// }

// impl AveragedCollection {
//     pub fn add(&mut self, value: i32) {
//         self.list.push(value);
//         self.update_average();
//     }
//     pub fn remove(&mut self) -> Option<i32> {
//         let result = self.list.pop();
//         // pop will return Optional<i32> value
//         match result {
//             Some(value) => {
//                 self.update_average();
//                 Some(value)
//             }
//             None => None,
//         }
//     }
//     pub fn average(&self) -> f64 {
//         self.average
//     }
//     // this is private method
//     fn update_average(&mut self) {
//         let total: i32 = self.list.iter().sum();
//         self.average = total as f64 / self.list.len() as f64;
//     }
// }
// /*Encapsulation that Hides Implementation Details
//  *  which means that the implementation details of an object aren’t accessible to code using that object.
//  * Therefore, the only way to interact with an object is through its "public API"; code using the object shouldn’t be able to reach into the object’s internals and change data or behavior directly.
//  * This enables the programmer to change and refactor an object’s internals without needing to change the code that uses the object.
//  */
// /***************************************************** */
