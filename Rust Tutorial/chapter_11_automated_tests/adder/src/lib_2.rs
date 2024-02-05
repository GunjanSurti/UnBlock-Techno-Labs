/**
 * You’ll put "unit tests" in the "src directory" in each file with the code that they’re testing
 * Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces. Integration tests are entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.
 * integration tests are entirely external to your library
 * The tests Directory
We create a tests directory at the top level of our project directory, next to src. 
Cargo knows to look for integration test files in this directory. 
We can then make as many test files as we want, and Cargo will compile each of the files as an individual crate.

Let’s create an integration test. With the code in Listing 11-12 still in the src/lib.rs file, make a tests directory, and create a new file named tests/integration_test.rs. Your directory structure should look like this:


project_name
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests 
    └── integration_test.rs
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
 *  
 * Each file in the tests directory is a separate crate, so we need to bring our library into each test crate’s scope. For that reason we add use adder at the top of the code, which we didn’t need in the unit tests.
 *
**************Submodules in Integration Tests*********************
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    ├── common
    │   └── mod.rs
    └── integration_test.rs

=> Filename: tests/integration_test.rs

use adder;

mod common; => though file name is mod.rs but we are importing folder name

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
 */