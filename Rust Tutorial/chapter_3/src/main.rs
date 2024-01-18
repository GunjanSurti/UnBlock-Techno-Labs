fn main() {
    let x = 5;

    let x = x + 1; // x = 6 this is called shadowing
                   // We can shadow a variable by using the same variable’s name and repeating the use of the let keyword

    {
        // this is called inner scope
        let x = x * 2; // x = 6 * 2 = 12
        println!("The value of x in the inner scope is: {x}");
    } // as the scope ends ... x = 6

    println!("The value of x is: {x}");
    // const Y: u8 = 7; // for const type is neccessary and value should be assigned
    let s = "      ";
    let s = s.len();
    print!("{s}");
}

// Compile-time constants, compile-time evaluable functions, and raw pointers.

// Compile-time constants
// Sometimes a certain value is used many times throughout a program, and it can become inconvenient to copy it over and over. What's more, it's not always possible or desirable to make it a variable that gets carried around to each function that needs it. In these cases, the const keyword provides a convenient alternative to code duplication:

// const THING: u32 = 0xABAD1DEA;

// let foo = 123 + THING;
// Constants must be explicitly typed; unlike with let, you can't ignore their type and let the compiler figure it out. Any constant value can be defined in a const, which in practice happens to be most things that would be reasonable to have in a constant (barring const fns). For example, you can't have a [File] as a const.

// The only lifetime allowed in a constant is 'static, which is the lifetime that encompasses all others in a Rust program. For example, if you wanted to define a constant string, it would look like this:

// const WORDS: &'static str = "hello rust!";
// Thanks to static lifetime elision, you usually don't have to explicitly use 'static:

// const WORDS: &str = "hello convenience!";
// const items looks remarkably similar to static items, which introduces some confusion as to which one should be used at which times. To put it simply, constants are inlined wherever they're used, making using them identical to simply replacing the name of the const with its value. Static variables, on the other hand, point to a single location in memory, which all accesses share. This means that, unlike with constants, they can't have destructors, and act as a single value across the entire codebase.

// Constants, like statics, should always be in SCREAMING_SNAKE_CASE.

// For more detail on const, see the Rust Book or the Reference.

// Compile-time evaluable functions
// The other main use of the const keyword is in const fn. This marks a function as being callable in the body of a const or static item and in array initializers (commonly called "const contexts"). const fn are restricted in the set of operations they can perform, to ensure that they can be evaluated at compile-time. See the Reference for more detail.

// Turning a fn into a const fn has no effect on run-time uses of that function.

// Other uses of const
// The const keyword is also used in raw pointers in combination with mut, as seen in *const T and *mut T. More about const as used in raw pointers can be read at the Rust docs for the [pointer primitive].

// The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime
// Rust’s naming convention for constants is to use all uppercase with underscores between words.

//  The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name.
//     let spaces = "   ";
//     let spaces = spaces.len();
// The first spaces variable is a string type and the second spaces variable is a number type. Shadowing thus spares us from having to come up with different names, such as spaces_str and spaces_num; instead, we can reuse the simpler spaces name. However, if we try to use mut for this, as shown here, we’ll get a compile-time error:
// Keep in mind that Rust is a statically typed language, which means that it must know the types of all variables at compile time.

// Data Types
// 1. Scalar Types 2. Compound

//1. Scalar:-  A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. You may recognize these from other programming languages. Let’s jump into how they work in Rust.
