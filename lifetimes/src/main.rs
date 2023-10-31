fn main() {
    // Validating References with Lifetimes

    // Every reference in Rust has a lifetime, which is the scope for which that
    // reference is valid. Most of the time, lifetimes are implicit and inferred,
    // just like most of the time, types are inferred. We must annotate lifetimes
    // when the lifetimes of referenes could be related in a few different ways.
    // Rust requires us to annotate the relationship using generic lifetime parameters
    // to ensure the actual references used at runtime will definitely be valid.

    // Annotating lifetimes is not a concept most other programming languages have,
    // so this is going to feel unfamiliar.

    // Preventing Dangling References with Lifetimes
    // The main aim of lifetimes is to prevent dangling references, which cause
    // a program to reference data other than the data it's intedted to reference.

    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);
    // How does Rust determine that this code is invalid? It uses a borrow checker.

    // The Borrow Checker
    // The Rust compiler has a borrow checker that compares scopes to determine
    // whether all borrows are valid.
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r);

    // Here, we've annotated the lifetime of r with 'a and the lifetime of x with 'b.
    // As you can see, the inner 'b block is much smaller than the ohter 'a lifetime
    // block. At compile time, Rust compares the size of the two lifetimes and sees
    // that r has lifetime of 'a but that it refers to memory with a lifetime of 'b.
    // The program is rejected because 'b is shorted than 'a: the subject of the
    // reference doesn't live as long as the reference.

    // Generic Lifetimes in Functions
    // We'll write a function that returns the longer of two string slices. This
    // function will take two string slices and return a single string slice. After
    // we've implemented the longest function.

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest is {}", result);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is: {}", result);
    }

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// We want the signature to express the following constraint: the returned reference
// will be valid as long as both the parameters are valid. This is the relationship
// between liefetimes of the parameters and the return value.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
