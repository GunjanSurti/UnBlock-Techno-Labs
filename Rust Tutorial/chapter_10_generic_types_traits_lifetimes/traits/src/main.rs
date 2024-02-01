use std::fmt::{Debug, Display};

mod more_traits;

#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    let tweet = Tweet {
        username: String::from("Gunjan Surti"),
        content: String::from("of course, as you probably already know, people"),
        reply: true,
        retweet: false,
    };

    // println!("1 new tweet: {}", tweet.summarize()); // => Gunjan Surti: Unblock
    // println!("Already implemented: {}", tweet.default()); //
    more_traits::more();
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.location, self.author)
    }
}
pub trait Summary {
    fn summarize(&self) -> String;
    // this method signature says that every implementation must implement this function in their own "impl struct"
    fn default(&self) -> String {
        String::from("This is default implementation...")
        // this is called default implementation so we dont need to
    }

    fn call_other_method(&self) {
        println!("{}", self.default()); // calling method of same trait
                                        // which help in calling multiple method using single method
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
/**************************Traits as Parameters******************************/

pub fn notify(item: &impl Summary) {
    // this Parameters accepts any type that implements the specific trait
    // here  we can call any method that comes with Summary trait
    println!("Breaking News! {}", item.summarize());
}

pub fn notify_2<T: Summary>(item: &T) {
    println!("Breaking News! {}", item.summarize());
}

pub fn n_3(_item1: &impl Summary, _item2: &impl Summary) {
    // here item1 and item2 can be different types as long as they implement same trait
}
// same as

pub fn n_4<T: Summary>(_item1: &T, _item2: &T) {
    // type of parameter passed must be same
}

/************************************Specifying Multiple Trait Bounds with the + Syntax***************************/
// Specifying more than one traits
#[allow(unused)]
pub fn n_5(item: &(impl Summary + Display)) {}

#[allow(unused)]
pub fn n_6<T: Summary + Display>(item: &T) {}

/************************using "Where clause" */
#[allow(unused)]
fn with_clause<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    14
}

/**********************************Returning Types that Implement Traits*****************************/
#[allow(unused)]
fn returns_summarizable() -> impl Summary {
    // here we are returning trait only
    // so we can return anything that implements "Summary"
    // But we can only return single type at a time
    // means, in "if else" statement...
    // if () {Tweet} else {NewsArticle} this will not work
    // only Tweet or NewsArticle
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

/***********************************************************************************************
A "trait" defines functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.

Note: Traits are similar to a feature often called "interfaces" in other languages, although with some differences.

A type’s behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types. Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

we declare the "method signatures" that "describe the behaviors" of the types that implement this trait,

Each type implementing this trait must provide its "own custom behavior" for the body of the method. compiler will enforce that any type that has the Summary trait will have the method summarize defined with this signature exactly.

A trait can have multiple methods in its body: the method signatures are listed one per line and each line ends in a semicolon.

we can implement "trait" on a "type" only when one of both is "local" means in "main/other crate"

But we can’t implement external traits on external types.

we can’t implement the Display trait on Vec<T> within our aggregator crate, because Display and Vec<T> are both defined in the standard library and aren’t local to our aggregator crate

**********************************Overriding*********************
we can Override defalut method by implementing in implementation of" Summary in Tweet"
*/
