fn main() {
    let a: i32 = 10;
    let b: i32 = 15;
    let c: i32 = 20;
    let x: f64 = std::f64::consts::PI;
    let _un: u8 = 10;
    let _i: i8 = 127;
    let _flt = 1.2;
    let _arr = [1, 2, 3];
    let arr2 = [100; 5]; // arr2 : [100, 100, 100, 100, 100]
    println!("Hello, world!, {} uygi {} {} {} ", a, b, c, x);
    println!("arr2 : {:?}, length {}", arr2, arr2.len());
}
