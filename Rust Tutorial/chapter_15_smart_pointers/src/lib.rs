// interior mutability pattern
// In this code we are tracking number of messager send (value) and there is a limts to send message (max)
pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
    // we have generic T which implements Messenger trait
    messenger: &'a T,
    value: usize, // total msg sent till not
    max: usize,   // max value of msg can be send
}
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell:: RefCell;
    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
        // vector of messages being sent
        // using RefCell bcz Messenger trait has send(&self,..) {&self} not '&mut self'
        // as we are changing message value we want mut but it is not given in library
        // so we are using interior mutability pattern (RefCell) to change value of immutable reference
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]), // empty vector
            }
        }
    }
    // implementing messenger trait for MockMessenger
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
            // sent_messages is RefCell smart pointer and its immutable but we can get mutable refrence to the value stored inside our smart pointer by calling "borrow_mut()", this will make mutable refrence
        }
    }
    #[test]
    fn it_sends_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        // borrow() bcz we only want to check length
    }
}

/*
 * the RefCell smart pointer checks the borrowing rules during runtime:
 * 1. we can’t have 2 mutable refrences at the same time
 * 2. References must always be valid.
 *
 * by combining Rc and RefCell smart pointer we can get multiple owners of mutable data
 *
 * enum List {
 *      Cons(Rc<RefCell<i32>>,Rc<List>),
 *      // RefCell => mutable , Rc => multiple owners
 *      Nil
 * }

*/
