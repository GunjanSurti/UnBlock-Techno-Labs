mod struct_program;
mod method_syntax;
#[derive(Debug)] // lets us print struct
struct User {
    active: bool,
    username: String, // &str is not acceptable
    email: String,
    sign_in_count: u64,
}
// It’s also possible for structs to store references to data owned by something else, but to do so requires the use of lifetimes

// Tuple Structs
struct Color(i32, i32, i32); // paranthesis
struct Point(i32, i32, i32);

// Unit-Like Structs without any fields
struct AlwaysEqual;
//No need for curly brackets or parentheses
// Imagine that later we’ll implement behavior for this type such that every instance of AlwaysEqual is always equal to every instance of any other type, perhaps to have a known result for testing purposes. We wouldn’t need any data to implement that behavior
fn main() {
    method_syntax::syntax();
    struct_program::prgm();
    let _subject = AlwaysEqual;
    // creating struct instance
    let user1 = User {
        active: true,
        username: String::from("asdf"),
        email: String::from("asdf@gmail.com"),
        sign_in_count: 1,
    };
    // println!("") will not work on struct
    // println!("{:?}",user1); // this will give formated struct
    // println!("{:#?}",user1); // this will give in one line

    // Note that the entire instance must be mutable
    // creating mutable struct
    let mut user2 = User {
        active: true,
        username: String::from("asdf"),
        email: String::from("asdf@gmail.com"),
        sign_in_count: 1,
    };
    // chaging email
    user2.email = String::from("qwer@gmail.com");
    // println!("{:#?}", user2);

    let _user3 = build_user(String::from("zxcv@gmail.com"), String::from("zxcv"));
    // println!("user3 = {:#?}", user3);

    let _user4 = User {
        // by using previous struct "user1" we are giving owner ship of "username" and "email" to user4
        // we will not be able to access both in user1
        // active and sign_in_count have copy trait so they remain to user1 and also assigned to user4
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // println!("User4 = {:#?}", user4);
    // println!("{:#?}",user1); // will not run
    //borrow of partially moved value: `user1`
    // partial move occurs because `user1.username` has type `String`, which does not implement the `Copy` trait

    // if we want to only have email different from user1 then..
    let _user5 = User {
        email: String::from("hjkl@gmail.com"),
        ..user2 // taking remaining values from user2  no comma needed
                // we can choose to specify values for as many fields as we want in any order, regardless of the order of the fields in the struct’s definition.
    };
    // println!("user5 = {:#?}", user5);
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    // Note that the black and origin values are different types because they’re instances of different tuple structs. Each struct you define is its own type, even though the fields within the struct might have the same types. For example, a function that takes a parameter of type Color cannot take a Point as an argument, even though both types are made up of three i32 values. Otherwise, tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a . followed by the index to access an individual value.
}

fn build_user(email: String, username: String) -> User {
    // returns User struct
    // Because the parameter names and the struct field names are exactly the same , we can use the "field init shorthand syntax" to rewrite build_user so it behaves exactly the same but doesn’t have the repetition of username and email
    User {
        active: true,
        email,
        username, // order of giving value does not matter
        sign_in_count: 1,
    }
}
