use std::thread;
use std::time::Duration;

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2)); // it will take 2 sec to run
//     intensity
// }
fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_intensity, simulated_random_number);
}
// in order to define struct, enums or functions we need to use generics and traits bound
struct Cacher<T>
where
    T: Fn(u32) -> u32,
    // Fn is a key word, "Function", one of the three
    // FuMut , FnOnce
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // new => is a constructor
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // this is closure
    // expensive_closure is storing closure itself nott num value
    // syntax to call closure => expensive_closure(intensity)
    // compiler can determint the type of closure based on first call on
    // even though closure is like function, it has access to variables which are in scope but not inside closure
    let mut cached_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today do {} pushup!", cached_result.value(intensity));
        println!("Next do {} situps!", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today!");
        } else {
            println!("Today run for {} minutes!", cached_result.value(intensity));
        }
    }
}

/***
* closure can capture variable in environment by 3 ways
* 1. taking ownership => FnOnce  => closures can't take ownership of one variable more than once
* 2. borrowing mutablly => FnMut
* 3. borrowing immutablly => Fn
*  Function tratis => FnOnce, FnMut, Fn
* Closures: Anonymous Functions that Capture Their Environment
* Ways to write closure
* fn  add_one_v1   (x: u32) -> u32 { x + 1 } => this is function
  let add_one_v2 = |x: u32| -> u32 { x + 1 };
  let add_one_v3 = |x|             { x + 1 };
  let add_one_v4 = |x|               x + 1  ;

 * If you want to force the closure to take "ownership" of the values it uses in the environment even though the body of the closure doesn’t strictly need ownership, you can use the "move keyword" before the parameter list.

 -----------------------------------------------------------------------

fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);     // this is closure

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}  
=> Before defining closure: [1, 2, 3]
After calling closure: [1, 2, 3, 7]
-----------------------------------------------------------------------
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}
Before defining closure: [1, 2, 3]
Before calling closure: [1, 2, 3]
From closure: [1, 2, 3]
After calling closure: [1, 2, 3]
*/
