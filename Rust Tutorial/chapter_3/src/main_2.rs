pub fn main_2() {
    // data type
    let p = -5 / 3; // Results in -1 real ans is -1.666666...
                    //Integer division truncates toward zero to the nearest integer.
    let o: f64 = 42.3 - 2.2; //40.099999999999994, more presice as f64
                             // let o: f32 = 42.3 - 2.2; => we used f32 so less presice
    println!("{p} {o} ");
    let i = 4 * 30;
    println!("{i}");

}
// 2 scalar type

// 2.1 Integer

// Additionally, the "isize and usize" types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
// 1_000 = 1000, "_" used as visual seperator
// Number literals	Example
// Decimal => 98_222
// Hex	   => 0xff
// Octal   => 0o77
// Binary  => 0b1111_0000
// Byte (u8 only)  => b'A'
// The primary situation in which you’d use "isize" or "usize" is when indexing some sort of collection.
// When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error.
// To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:

// Wrap in all modes with the wrapping_* methods, such as wrapping_add.
// Return the None value if there is overflow with the checked_* methods.
// Return the value and a boolean indicating whether there was overflow with the overflowing_* methods.
// Saturate at the value’s minimum or maximum values with the saturating_* methods.

// 2.2 Floating-Point Types

// two types => f32, f64
// let x = 2.0; // f64

//     let y: f32 = 3.0; // f32

// which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.
// The f32 type is a single-precision float, and f64 has double precision.

// 2.3 boolean => true, false
// 2.4 character type

// let c = 'z';
// let z: char = 'ℤ'; // with explicit type annotation
// let heart_eyed_cat = '😻';

// Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes. Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust. Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust. We’ll discuss this topic in detail in “Storing UTF-8 Encoded Text with Strings” in Chapter 8.