/******************************* Product code ******************************************/

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

/******************************** Test code ******************************************/
#[cfg(test)]
// => tells to rust compiler that only compile code when "cargo test"
// configuration = test
// for unit test only
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
    fn smaller_cannot_hold_larger() {
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

/*
* cargo test => command to run tests
* assert!() => true ot false
* assert_eq!(a,b) => assert equal, arguments can be interchanged
* assert_ne!(a,d) => assert not equal, arguments can be interchanged
* Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively
*  For structs and enums that you define yourself, you’ll need to implement PartialEq to assert equality of those types. You’ll also need to implement Debug to print the values when the assertion fails. Because both traits are derivable traits, as mentioned in Listing 5-12 in Chapter 5, this is usually as straightforward as adding the #[derive(PartialEq, Debug)] annotation to your struct or enum definition
*assert!(result.contains("Carol")); => contains => says it contain or not
 We place the "#[should_panic]"" attribute after the #[test] attribute
 if we want to make function a panic..
*$ cargo test -- --test-threads=1 => to use only one thread
* as each test takes its own thread to run in parallel
* $ cargo test -- --show-output => as println! will not work
* cargo run test_name => to run single test
* cargo run module_name =>running test by module
* to ignore test add => "#[ignore]" Below #[test]
*cargo test -- --ignored => to run ignored test only
* cargo test -- --include-ignored => to run all test
*/

/*
  * mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")] => by adding expected and a string which says about bug
    -> this says that we are expecting that the panic message should be same as above or else it will throw error
    fn greater_than_100() {
        Guess::new(200);
    }
}

  */

/*
  Using Result<T, E> in Tests
Our tests so far all panic when they fail. We can also write tests that use Result<T, E>! Here’s the test from Listing 11-1, rewritten to use Result<T, E> and return an Err instead of panicking:

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
The "it_works" function now has the "Result<(), String>" return type. In the body of the function, rather than calling the assert_eq! macro, we return Ok(()) when the test passes and an Err with a String inside when the test fails.

Writing tests so they return a Result<T, E> enables you to use the "?" (question mark) operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.

can’t use "#[should_panic]" annotation on tests that use "Result<T, E>".
To assert that an operation returns an "Err variant", don’t use the question mark operator on the Result<T, E> value. Instead, use assert!(value.is_err()).

 */
