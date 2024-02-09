use add_one; // using "_" instead of "-"
use rand::{self, Rng};
fn main() {
    let num = 10;

    println!("Hello World!, {} plus one is {} ", num, add_one::add_one(num));
    let mut ers = rand::thread_rng();
    let es = ers.gen_range(-100..100);
    println!("{:#?}",es);
}
// $ cargo run -p adder => run this to "run" specific package 
// -p => package and then package name 
