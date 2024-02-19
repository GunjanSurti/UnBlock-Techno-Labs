pub fn slice_type() {
    let st = String::from("Hello World");
    let hello = &st[0..5]; // hello, at 5 is space which is not consider
    let world = &st[6..]; // 6 to till end
                          // [0..5] = [..5]
    println!("{hello} {world}");
    let fw = first_word(&st);
    
    println!(" First word : {fw}");
}

fn first_word(s: &String) -> &str {
    let byte = s.as_bytes();
    for (i, &item) in byte.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
/*
Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.
We create slices using a range within brackets by specifying [starting_index..ending_index], where starting_index is the first position in the slice and ending_index is one more than the last position in the slice
*/

// fn main() {
//     let mut s = String::from("hello world");

//     let word = first_word(&s);

//     s.clear(); // error! as refrence is cleared so "word " becomes invalid
        // as it is immutable refrence

//     println!("the first word is: {}", word);
// }

// This is also why string literals are immutable; &str is an immutable reference.