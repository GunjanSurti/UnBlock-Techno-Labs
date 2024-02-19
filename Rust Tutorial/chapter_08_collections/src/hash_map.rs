use std::collections::HashMap;

pub fn hash_map() {
    // println!("Hello Hash-Map");

    let mut hm = HashMap::new();

    hm.insert("red", 10);
    hm.insert("blue", 5);

    // let mut h = HashMap::new();
    // h.insert(5, "hi");

    /***********************************Accessing Values in a Hash Map ******************************/

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    // Here, "score" will have the value that’s associated with the Blue team, and the result will be 10. The "get method returns an Option<&V>"; if there’s no value for that key in the hash map, get will return None. This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, then "unwrap_or" to set score to zero if scores doesn't have an entry for the key.
    println!("{score}"); // => 10

    for (key, value) in &hm {
        println!("k={key} v={value}");
        // k=red v=10
        // k=blue v=5
    }

    // For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // after this line  field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // If we insert references to values into the hash map, the values won’t be moved into the hash map. The values that the references point to must be valid for at least as long as the hash map is valid.

    /***************************Updating a Hash Map*********************************/
    // cases => 1. replace the old value with the new value, completely disregarding the old value.
    //          2.keep old value & ignore the new value,only adding the new value if the key doesn’t already have a value.
    //          3. you could combine the old value and the new value.

    // Case 1 => Overwriting a Value
    // when we add same key, the value associated with that key will be replaced

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25);

    // println!("{:?}", scores); => 25 value is overwritten

    // Case 2 => Adding a Key and Value Only If a Key Isn’t Present
    //if the key does exist in the hash map, the existing value should remain the way it is.
    // If the key doesn’t exist, insert it and a value for it.

    // scores.insert(String::from("Blue"), 10);
    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value. This technique is much cleaner than writing the logic ourselves and, in addition, plays more nicely with the borrow checker.
    // Running the code in Listing 8-24 will print {"Yellow": 50, "Blue": 10}. The first call to entry will insert the key for the Yellow team with the value 50 because the Yellow team doesn’t have a value already. The second call to entry will not change the hash map because the Blue team already has the value 10.
    
    // Case 3 => Updating a Value Based on the Old Value
    
    let text = " Hello World! Hello  Me";

    let mut hmap = HashMap::new();

    for c in text.split_whitespace() {
        let count = hmap.entry(c).or_insert(0);
        *count += 1;
    }
    println!("{:?}",hmap); // {"Hello": 2, "Me": 1, "World!": 1}
    // The "split_whitespace" method returns an "iterator" over sub-slices, separated by whitespace, of the value in text. The or_insert method returns a "mutable reference" "(&mut V)" to the value for the specified key. Here we store that mutable reference in the count variable, so in order to assign to that value, we must first "dereference" count using the asterisk (*). The mutable reference goes out of scope at the end of the for loop, so all of these changes are safe and allowed by the borrowing rules.
}

// The type "HashMap<K, V>" stores a mapping of keys of type K to values of type V using a hashing function
// there should be same type of keys in a hash map and same type of values
// we cannot have different types of keys and values
// hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type.

/*
hm.get => fn(&self, &Q) -> Option<&V>

Returns a reference to the value corresponding to the key.

The key may be any borrowed form of the map's key type, but [Hash] and [Eq] on the borrowed form must match those for the key type.
*/
