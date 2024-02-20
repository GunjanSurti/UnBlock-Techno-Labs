fn main() {
    let answer = do_twice(add_one, 5);
    // println!("{answer}");

    let list_of_number: Vec<i32> = vec![1, 2, 3, 4, 5];
    let list_of_strings: Vec<String> = list_of_number.iter().map(|i| i.to_string()).collect();
    // let list_of_strings: Vec<String> = list_of_number.iter().map(ToString::to_string).collect();
    // println!("{:?}", list_of_strings);

    let am = 0i32..20; // this is range from 0 to 20 in "i32" type
    println!("{:#?}", am);
    #[derive(Debug)]
    enum Status {
        Value(i32),
        Stop,
        // the name of each enum variant that we define also becomes an "initializer function". We can use these initializer functions as "function pointers" that implement the "closure traits", which means we can specify the initializer functions as arguments for methods that take closures
    }
    // Here we create "Status::Value" instances using each u32 value in the range that map is called on by using the initializer function of Status::Value. Some people prefer this style, and some people prefer to use closures
    let list: Vec<Status> = (0i32..20).map(Status::Value).collect();
    println!("{:?}", list)
}

fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    // this funtion takes a function as a parameter
    //this is "function pointer"
    f(arg) + f(arg)
}

fn twice_do<T>(f: T, args: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    // this is "closure trait bound(Fn)"
    f(args) + f(args)
}

// this function returns closure
fn return_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn return_diff_closure(a: i32) -> Box<dyn Fn(i32) -> i32> {
    if a > 0 {
        Box::new(move |b| a + b)
    } else {
        Box::new(move |b| a - b)
    }
}
/*

Function pointers are pointers that point to code, not data.
assumed to not be null

Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), meaning you can always pass a function pointer as an argument for a function that expects a closure
*/
