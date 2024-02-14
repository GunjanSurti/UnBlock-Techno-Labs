pub struct Post {
    content: String,
}
pub struct DraftPost {
    content: String,
}
impl Post {
    pub fn new() -> DraftPost {
        // maikng sure only DraftPost is created first
        DraftPost {
            content: String::new(),
        }
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}
impl PendingReviewPost {
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
/*******************************************By OOPs *************************************
pub struct Post {
    state: Option<Box<dyn State>>,
    // Box<dyn State> => trait object it’s a stand-in for any type inside a Box that implements the "State" trait.
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            // we are ensuring that when a post is created it will always be a Draft
            state: Some(Box::new(Draft {})),

            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        // this function dosent dependon the state of the post is in therefoer its not part of state pattern
        // as add_text method dosent interract with "state field"
        self.content.push_str(text);
    }
    pub fn content(&self) -> &str {
        // here we will check whether the post is in "which" state
        // ""
        self.state.as_ref().unwrap().content(self)
        // self.state => Option that owns State Object
        // as_ref => give refrence
        // unwrap() => will give value inside of the Some variant || as we know there is a valid state variant every time
    }
    // this function takes mutable refrence so that we can change state value
    pub fn request_review(&mut self) {
        // take() => Takes the "value out of the option", leaving a "None" in its place
        if let Some(state) = self.state.take() {
            // if the pattern matches then it will go in this block
            self.state /* None  */ = Some(state.request_review()); // assigning Some()
        }
    }
    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approved())
        }
    }
}
// State trait define shared behaviour between various statte of a post
// currently Draft only but later Pending Review and publish will be added
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approved(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
        // this method wil be default implementation
        // we can override this method in particular implementation
    }
}

struct Draft {}

// we are implementing "State trait" for "Draft struct"
impl State for Draft {
    // request_review will take ownership of a Box containing self
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // as self will take ownership it will be invalid and we are returning new state that can be used in its place
        Box::new(PendingReview {})
    }
    // we are returning self bcz we cant approve a draft until a review has been requested
    fn approved(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    // here we dont need to anything as it will already be in pending review state, after draft state
    // so we are returning self
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approved(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    // approving a Published state has no effect
    fn approved(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        // our life time of return type is tied to the life time of post
        &post.content
    }
}
*/
/***
 *
 * This Rust code is using pattern matching with an `Option` type to manipulate the `self.state` field. Here's a breakdown of what it does:

1. `self.state.take()`: This method extracts the value from the `self.state` field and replaces it with `None`. It moves the value out of `self.state`, leaving it empty.

2. `if let Some(state) = ...`: This is a pattern matching construct in Rust. It checks whether the result of `self.state.take()` is `Some`, which means it contains a value. If it does, the value is bound to the variable `state` within the scope of the `if` block.

3. `self.state = Some(state.request_review());`: Inside the `if let` block, the `request_review()` method is called on the `state` variable, which presumably returns a modified version of the state. This modified state is then wrapped in `Some` and assigned back to `self.state`.

Here's what happens step by step:

- If `self.state` is `Some`, it extracts the value and assigns it to `state`.
- Then, it calls `state.request_review()`, presumably mutating the state in some way.
- Finally, it assigns the modified state back to `self.state` wrapped in `Some`.

This code pattern is often used in Rust when you need to temporarily take ownership of a value, perform some operation on it, and then put it back in place, potentially with modifications. It's a common idiom for state management and mutation in Rust.
 */
