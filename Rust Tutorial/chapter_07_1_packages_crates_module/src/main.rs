mod garden;

// mod garden;
// The compiler will look for the module’s code in these places:
// 1. Inline, within curly brackets that replace the semicolon following mod garden
// 2. In the file src/garden.rs
// 3. In the file src/garden/mod.rs
fn main() {
    println!("Hello, world!");
}
// when we type "cargo new .. " it creates package 
// package stores creates
// crates => 1. binary => code that can be executed (code in main.rs file etc) 
//           2. library => code that can be used by other programs 

// https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html
