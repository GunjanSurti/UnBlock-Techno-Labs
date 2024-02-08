pub trait Iterator {
    // the Iterator's methods is global only one method we need to implement
    // associated type
    type Item;
    // returns next in line in iterator
    fn next(&mut self) -> Option<Self::Item>;
    // this next() method we have to implement to methods we use this "Iterator" for
}

#[test]
fn iter_demo() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iter_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();
    // .sum() will call next repeateadly and add all numbers
    assert_eq!(total, 6);
}

pub fn iterate() {
    // let v1 = vec![1, 2, 3, 4];
    // let v1_iter = v1.iter();

    // for item in v1.iter()
    // /*v1_iter*/
    // {
    //     println!("Got {}", item);
    // }
    // iter_map();
}

#[allow(dead_code)]
pub fn iter_map() {
    let v1 = vec![1, 2, 3];
    // collect() is a consumer method which will take our iterator and tranfroms to collections
    // map() returns iterators
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

/***
 * iter() => immutable refrences
 * iter_mut() => mutable
 * into_iter() => owned types
 *
 * types of iterator provided by standard library
 * adapter => takes iterator and return other iterator
 *consumers => takes iterator and return  different data type
 */
