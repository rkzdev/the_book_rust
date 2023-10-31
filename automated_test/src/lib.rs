// How to Write Test

// 1. Set up any needed data or state.
// 2. Run the code you want to test.
// 3. Assert the results are what you expect.

// Test Organization
// As mentioned at the start of the chapter, testing is a complex discipline,
// and different people use different terminology and organization. The Rust
// community thinks about test in terms of two main categories: unit tests and
// integration tests. Unit tests are small and more focused, testing one module
// in isolation at a time, and can test private interfaces. Integration tests are
// entirely external to your library and use your code in the same way any other
// external code would, using only the public interface and potentially exercising
// multiple modules per test.

// Writing both kinds of test is important to ensure that the piece of your library
// are doing what you expect them to, separately and together.

// Unit Tests
// The purpose of unit tests it to test each unit of code in isolation from the
// rest of the code quickly pinpoint where code is and isn't working as expected.
// You'll put unit tests in the src directory in each file with the code that
// they're testing. The convention is to create a module named tests in each file
// to contain the test functions and to annotate the module with cfg(test).

// The Tests Module and #[cfg(test)]
// The #[cfg(test)] annotation on the tests module tells Rust to compile and run
// the test code only when you run cargo test, not when you run cargo build. This
// saves compile time when you only want to build the library and saves space in
// the resulting compiled artifact because the tests are not included. You'll see
// that because integration tests go in a different directory, they don't need the
// #[cfg(test)] annotation. However, because unit tests go in the same files as the
// code, you'll use #[cfg(test)] to specify that they shoudn't be included in the
// compiled result.

// Testing Private Functions
// There's debate within the testing community about whether or not private functions
// should be tested directly, and other languages make it difficult or impossible
// to test private functions. Regardless of which testing ideology you adhere to,
// Rust's privacy fules do allow you to test private functions.
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}
// Note that the interal_adder function is not marked as pub. Tests are just Rust
// code, and the tests module is just another module. items in child modules can
// use the items in their ancestor modules. In this tes, we bring all of the test
// module's parent's items into scope with use super::*, and then the test can
// call internal_adder. If you don't think private functions should be tested,
// there's nothing in Rust that will compel you to do so.

// Integration Tests
// In Rust, integration tests are entirely external to your library. They use your
// library in the same way any other code would, which means they can only call
// functions that are part of your library's public API. Their purpose is to test
// whether many parts of your library work together correctly. Units of code that
// work correctly on their own could have problems when integrated, so test coverage
// of the integrated code is important as well. To create integration tests, you
// first need a tests directory.

// The tests Directory
// We create a tests directory at the top level of our project directory, next to
// src. Cargo knows to look for integration test files in this directory. We can
// then make as many test files as we want, and Cargo will compile each of the files
// as an individual crate.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    // format!("Hello {}", name)
    String::from("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value msut be greater than or equal to 1, got {}",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol");
    //     assert!(
    //         result.contains("Carol"),
    //         "Greeting did not contain name, value was `{}`",
    //         result
    //     );
    // }

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        //
    }
}
