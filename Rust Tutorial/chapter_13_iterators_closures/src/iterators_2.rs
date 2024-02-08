pub fn iter_2() {}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

#[allow(dead_code)]
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // here in parameter we are taking ownership of "Shoe"
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    // "into_iter()" => creates iterator that takes ownership of our vector
    // "filter()" => accepts a closure which will be called for "every element" in our iterator created by "into_iter"
    // filter creates an iterator, if closure results in true for specific element then it will be "included" in "resulting iterator"
    // finally "collect()" => which will take terator and transform into collection in this our collection will be vector of Shoes
}

struct Counter {
    count: u32,
}
#[allow(dead_code)]
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("Sneakers"),
            },
            Shoe {
                size: 13,
                style: String::from("Sandals"),
            },
            Shoe {
                size: 10,
                style: String::from("Boots"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("Sneakers"),
                },
                Shoe {
                    size: 10,
                    style: String::from("Boots"),
                }
            ]
        )
    }

    #[test]
    fn calling_next_directly() {
        let mut ctr = Counter::new(); // ctr = 0

        assert_eq!(ctr.next(), Some(1)); // after next it will become 1
        assert_eq!(ctr.next(), Some(2));
        assert_eq!(ctr.next(), Some(3));
        assert_eq!(ctr.next(), Some(4));
        assert_eq!(ctr.next(), Some(5));
        assert_eq!(ctr.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_method() {
        let sum: u32 = Counter::new() /* 1st iterator */
            .zip(Counter::new().skip(1) /* 2nd iterator */)
            .map(
                |(a, b)| a * b, /* take value from 2 iterators in tuple form */
            )
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(sum, 18);
    }
}

/*****************************Iterator Trait Methods*************************/

/**********************************Map********************************
 *
 * Takes a closure and creates an iterator which calls that closure on each element.

map() transforms one iterator into another, by means of its argument: something that implements FnMut. It produces a new iterator which calls this closure on each element of the original iterator.

If you are good at thinking in types, you can think of map() like this: If you have an iterator that gives you elements of some type A, and you want an iterator of some other type B, you can use map(), passing a closure that takes an A and returns a B.

map() is conceptually similar to a for loop. However, as map() is lazy, it is best used when you're already working with other iterators. If you're doing some sort of looping for a side effect, it's considered more idiomatic to use for than map().
 *
 * ************************************Skip*********************************
 *
 * Creates an iterator that skips the first "n elements".

skip(n) skips elements until n elements are skipped or the end of the iterator is reached (whichever happens first). After that, all the remaining elements are yielded. In particular, if the original iterator is too short, then the returned iterator is empty.

Rather than overriding this method directly, instead override the nth method.
 *
 ***********************************Zip***********************************************
 *
 * 'Zips up' two iterators into a single iterator of pairs.

zip() returns a new iterator that will iterate over two other iterators, returning a tuple where the first element comes from the first iterator, and the second element comes from the second iterator.

In other words, it zips two iterators together, into a single one.

If either iterator returns "None", [next] from the zipped iterator will return None. If the zipped iterator has no more elements to return then each further attempt to advance it will first try to advance the first iterator at most one time and if it still yielded an item try to advance the second iterator at most one time.

To 'undo' the result of zipping up two iterators, see [unzip].

 * ************************************Filter*********************************
Creates an iterator which uses a closure to determine if an element should be yielded.

Given an element the closure must return "true or false". The returned iterator will yield only the elements for which the closure returns true.

 * ************************************Sum*********************************

 Sums the elements of an iterator.

Takes each element, adds them together, and returns the result.

An empty iterator returns the zero value of the type.

sum() can be used to sum any type implementing Sum, including Option and Result.

 Panics
When calling sum() and a primitive integer type is being returned, this method will panic if the computation overflows and debug assertions are enabled.
*/
