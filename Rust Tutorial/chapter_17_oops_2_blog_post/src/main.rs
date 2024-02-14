use chapter_17_oops_2_blog_post::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    // shadowing
    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
/*
 *  Object-oriented patterns won’t always be the best solution in Rust due to certain features, like ownership, that object-oriented languages don’t have.
 * 
 * though Rust is capable of implementing object-oriented design patterns, other patterns, such as encoding state into the type system, are also available in Rus
 */

/*    by using Object Orented programing
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content()); // post is in draft state so it is empty

    post.request_review();
    assert_eq!("", post.content()); // post is in draft state so it is empty

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content()); // as "approved"

}
*/
