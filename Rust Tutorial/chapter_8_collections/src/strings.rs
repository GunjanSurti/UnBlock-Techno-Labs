#![allow(unused_imports)]
use std::fmt::format;
#[allow(dead_code)]
pub fn storing_strings() {
    // println!("Hello Strings");``

    // let mut s1 = String::from("Hello ");
    // s1.push_str("World");
    // s1.push('!');
    // println!("{s1}")

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("World!");
    // let _s3 = format!("{}{}", s1, s2);
    // "format!" doesnt take "ownership" so we can reuse s1,s2
    // let s6 = format!("{}{}{}", s1, s2, _s3);
    // let s4 = "gunjan";
    // let _s5 = &s4.to_string();
    // let _s5 = "kjd";
    // println!(" 5 = {_s5}");
    // // let s7 = s.push_str(s4);

    /******************************Appending to a String with push_str and push *********************************/
    // let mut s = String::from("Gunjan "); // this should be mutable
    // s.push_str("Surti"); // this is "str" not String
    // "push_str" dont take ownership
    // println!("{s}");
    // s.push('K.') // uesed to push single character
    // let mut s = String::from("lo");
    // s.push('l'); s = lol

    /*******************Concatenation with the + Operator or the format! Macroc ****************************/

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;  // "+ &s2" refrence so can use again
    // means we cant add to "String" together only "String + str" ,
    //  here &2 is  String but rust some things (deref coercion)
    // note s1 has been moved here and can no longer be used
    //  The + operator uses the add method, whose signature looks something like this:
    // fn add(self, s: &str) -> String {

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    // // s => tic-tac-toe
    let s1 = String::from("tic");

    // another way
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    /*******************************Indexing into Strings********************************* */
    // if you try to access parts of a String using indexing syntax in Rust, you’ll get an error
    // A String is a wrapper over a Vec<u8>

    // "Bytes" and "Scalar Values" and "Grapheme Clusters"!
    // Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).
    //     If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values that looks like this:

    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // That’s 18 bytes and is how computers ultimately store this data. If we look at them as Unicode scalar values, which are what Rust’s char type is, those bytes look like this:

    // ['न', 'म', 'स', '्', 'त', 'े']
    // There are six char values here, but the fourth and sixth are not letters: they’re diacritics that don’t make sense on their own. Finally, if we look at them as grapheme clusters, we’d get what a person would call the four letters that make up the Hindi word:

    // ["न", "म", "स्", "ते"]

    let hello = "Здравствуйте";

    let s = &hello[0..4]; // => Зд its 3A same on string just assume that, 0 to 4 Bytes
                          // we mentioned that each of these characters was 2 bytes,
    println!("{s}");

    // For individual Unicode scalar values, use the chars method. Calling chars on “Зд” separates out and returns two values of type char, and you can iterate over the result to access each element:

    for c in "Зд".chars() {
        println!("{c}");
        // З
        // д
    }
    for b in "Зд".bytes() {
        println!("{b}");
        // op ->  208
        //        151
        //        208
        //        180
        // But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.
    }
}

// Strings are stored as collection of UTF-8 encoded bytes
// "strings" are implemented as a "collection of bytes", plus some methods to provide useful functionality when those bytes are interpreted as text.

// Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str. string slices, which are references to some UTF-8 encoded string data stored elsewhere. String literals, for example, are stored in the program’s binary and are therefore string slices.

/*/
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
*/
