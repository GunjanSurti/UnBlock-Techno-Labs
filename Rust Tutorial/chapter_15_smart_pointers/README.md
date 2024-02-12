# Notes

A pointer is a general concept for a variable that contains an address in memory. This address refers to, or “points at,” some other data. The most common kind of pointer in Rust is a reference

References are indicated by the & symbol and borrow the value they point to. They don’t have any special capabilities other than referring to data, and have no overhead.

"Smart pointers", are data structures that act like a pointer but also have additional metadata and capabilities

"reference counting" smart pointer type. This pointer enables you to allow data to have multiple owners by keeping track of the number of owners and, when no owners remain, cleaning up the data.

"references" only borrow data, in many cases, "smart pointers" own the data they point to.

Smart pointers => eg. String and Vec<T> (they own some memory and allow you to manipulate it)
More examples:
1.Box<T> for allocating values on the heap
2.Rc<T>, a reference counting type that enables multiple ownership
3.Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

Smart pointers are usually implemented using structs.

Unlike an ordinary struct, smart pointers implement the Deref and Drop traits.

The Deref trait allows an instance of the smart pointer struct to behave like a reference so you can write your code to work with either references or smart pointers.

The Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope

<!---------------------------------- Using Box<T> to Point to Data on the Heap---------------------------------->

The most straightforward smart pointer is a box, whose type is written Box<T>.

"Boxes" allow you to "store data" on the "heap" rather than the "stack". What remains on the stack is the pointer to the heap data.

You’ll use them most often in these situations:

1. When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
=> “Enabling Recursive Types with Boxes” section

2. When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
=>In the second case, transferring ownership of a large amount of data can take a long time because the data is copied around on the "stack". To improve performance in this situation, we can store the large amount of data on the "heap" in a box. Then, only the small amount of "pointer data" is copied around on the stack, while the data it references stays in one place on the heap.

3. When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
=> The third case is known as a "trait object", and Chapter 17 devotes an entire section, “Using Trait Objects That Allow for Values of Different Types,” just to that topic. So what you learn here you’ll apply again in Chapter 17!
