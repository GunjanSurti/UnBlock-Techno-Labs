pub fn storing_strings() {
    println!("Hello Strings");

    // let mut s1 = String::from("Hello ");
    // s1.push_str("World");
    // s1.push('!');
    // println!("{s1}")

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = format!("{}{}", s1, s2); // "format!" doesnt take ownership so we can reuse s1,s2

    let s4 = "gunjan";
    let s5 = &s4.to_string();
    let s5 = "kjd";
}

// Strings are stored as collection of UTF-8 encoded bytes
// "strings" are implemented as a "collection of bytes", plus some methods to provide useful functionality when those bytes are interpreted as text.

// Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str. string slices, which are references to some UTF-8 encoded string data stored elsewhere. String literals, for example, are stored in the program’s binary and are therefore string slices.