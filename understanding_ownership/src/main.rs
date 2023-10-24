#[allow(unused_variables)]
fn main() {
    // The stack and the Heap

    // The stack stores values in the order it gets them and removes the values in the opposite
    // order. This is referred to as last in, first out (LIFO). Adding data is called pushing onto
    // the stack, and removing is called popping off the stack.
    // All data stored on the stack must have a known, fixed size.
    // Data with an unknown size at compile time or a size that might change must be stored on the
    // heap instead.

    // The heap is less orgnaized: when you put data on the heap, you request a certain amount of
    // space. The memory allocator finds an empty spot in the heap that is big enough, marks it as
    // being in use, and returns a pointer, which is the address of that location. This process is
    // called allocating on the heap.

    // Ownership Rules
    // - Each value in Rust has an owner.
    // - There can only be one owner at a time.
    // - When the owner goes out of scope, the value will be dropped.

    // Variable Scope

    // s is not valid here, it's not yet declared
    {
        // s is valid from this point forward
        let s = "hello";
    } // this scope is now over, and s is no longer valid

    // The String Type
    // We'll concentrate on the parts of String that relate to ownership. These aspect also apply
    // to other complex data types, whether they are provided by the standard library or created by
    // you.

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // So, what's the difference here? Why can String be mutated but literals cannot? The
    // difference is how these two types deal with memory.

    // Memory and Allocation
    // In the case of a string literal, we know the contents at compile time, so the text is
    // hardcoded directly into the final executable. This is why string literal are fast and
    // efficient. But thesse properties only comes from the string literal's immutability.
    // Unfortunately, we can't put blob of memory into the binary for each piece of text whose size
    // is unknown at compile time and whose size might change while running the program.

    // With the String type, in order to support a mutable, growable piece of text, we need to
    // allocate an amount of memory on the heap, unknown at compile time to hold the contents.
    // This means:
    // - The memory must be requested from the memory allocator at runtime.
    // - We need a way of returning this memory to the allocator when we're done with our String.

    // Rust takes a different path: the memory is automatically returned once the variable that
    // owns it goes out of scope. When a variable goes out of scope, Rust call a special function
    // for us. This function is called "drop". Rust calls drop automatically at the closing curly
    // bracket.

    // Variable and Data Interacting with Move

    // Because integers are simple values with a known, fixed size, and these two 5 values are
    // pushed on the stack.
    // let x = 5;
    // let y = x;
    // println!("x is: {x}, y is: {y}");

    // When we assign s1 to s2, the String data copied, meaning we copy the pointer, the length and
    // the capacity that are on the stack. We do not copy the data on the heap that the pointer
    // refers to.
    // Freeing memory twice is called "double free" error and it can lead to memory corruption,
    // which can lead to security vulnerabilities.

    // If you've heard the terms shallow copy and deep copy while working with other languages, the
    // concept of copying the pointer, length, and capacity without copying the data probably
    // sounds like making a shallow copy. But because Rust also invalidates the first variable,
    // instead of being called a shallow copy, it's known as a "move".

    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("s1: {s1}, s2: {s2}");

    // Variables and Data Interacting with clone
    // If we do want to deeply copy the head data of the String, not just the stack data, we can
    // use common method called clone.

    // When you see a call to clone, you know that some arbitrary code is being executed and that
    // code may be expensive. It's a visual indicator that something different is going on.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1: {s1}, s2: {s2}");

    // Stack-Only Data: Copy
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    // But this code seem to contradict what we just learned: we don't have a call to clone, but x
    // is still valid and wasn't moved into y.

    // The reason is that types such as integers that have a known size at compile time are stored
    // entirely on the stack, so copies of the actual values are quick to make. That means there's
    // no reason we would want to prevent x from being valid after we create the variable y.

    // rust has a special annotation called the Copy trait that we can place on types that are
    // stored on the stack, as integers are. If a type implements the Copy trait, variables that
    // use it do not move, but rather are trivially copied, making them still valid after
    // assignment to another variable.

    // Rust won't let us annotate a type with Copy if the type, or any of its parts, has
    // implemented the Drop trait. We'll get a compile-time error.

    // Here are some types that implement Copy:
    // - All the integer types, such as u32
    // - The boolean type, bool, with values true and false.
    // - All the floating-point type such as f64.
    // - The character type, char.
    // - Tuples, if they only contain types that also implement Copy. (i32, i32)
    // but (i32, String) does not.

    ownership_and_functions();
    returning_values_and_scope();
    references_and_borrowing();
    dangling_referenes();

    // The Rules of References
    // - At any given time, you can have either one mutable reference or any number of immutable
    // references.
    // - References must always be valid.

    the_slice_type();
}

fn ownership_and_functions() {
    // The machanism of passing a value to a function are similar to those when assigning a value
    // to a variable. Passing a variable to a function will move or copy, just as assignment does.

    let s = String::from("hello");

    takes_ownership(s);
    // println!("s = {}", s);

    // If we tried to use s after the call to takes_ownership, Rust would throw a compile-time
    // error. These static checks protet us from mistakes.
    let x = 5;
    makes_copy(x);
    println!("x = {}", x);
}

#[allow(unused_variables)]
fn returning_values_and_scope() {
    // Returning values can also transfer ownership

    // gives_ownership moves its return value to s1
    let s1 = gives_ownership();

    // s2 comes into scope
    let s2 = String::from("hello");

    // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    let s3 = takes_and_gives_back(s2);
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nthing happens. s1 goes out of
  // scope and is dropped.

#[allow(unused_variables)]
fn references_and_borrowing() {
    // While this works, taking ownership and then returningg ownership with every function is a
    // bit tedious. What if we want to let a function use a value but not take ownership?

    // The issue with this code is that we have to return the String to the calling function so we
    // can still use the String after the call to tedious_calulate_length.
    let s1 = String::from("hello");
    let (s2, len) = tedious_calculate_length(s1);
    println!("The length os '{}' is {}.", s2, len);

    // Instead, we can provide a reference to the String value. A reference is like a pointer in
    // that it's an address we can follow to access the data stored at that address; that data is
    // owned by some other variable. Unlike a pointer, a referene is guaranteed to point to a valid
    // value of a particular type for the life of that referene.
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // First, notice that all the tuple code in the variable declaration and the function return
    // value is gone. Second, note that we pass &s1 into calculate_length and, in its definition,
    // we take &String rather than String. These ampersands represent references, and they allow
    // you to refer to some value without taking ownership of it.

    // Note: The opposite of referencing by using & is dereferencing, which is accomplised with the
    // dereference operator, *.

    // So, what happens if we try to modify something we're borrowing?
    // Just as variables are immutable by default, so are references. We're not allowed to modify
    // something we have a reference to.

    // Mutable Referenes

    let mut s = String::from("hello");
    change(&mut s);
    println!("The value of s now is '{}'", s);

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    // Mutable references have on big restriction: if you have a mutable reference to a value, you
    // can have no other references to that value. This code that attempts to create to mutable
    // references to s will fail

    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // This error says that this code is invalid because we cannot borrow s as mutable more than
    // once at a time. The first mutable borrow is in r1 and must last until it's used in the
    // println!, but between the creation of that mutable reference and its usage, we tried to
    // create another mutable reference in r2 that borrows the same data as r1.

    // The restriction preventing multiple mutable references to the same data at the same time
    // allows for mutation but in a very controlled fashion. It's something that new Rustaceans
    // struggle with because most languages let you mutate whenever you'd like. The benefit of
    // having this restriction is that Rust can prevent data races at compile time. A data race is
    // similar to a race condition and happens when these three behaviors occurs:

    // - Two or more pointers access the same data at the same time.
    // - At least one of the pointers is being used to write to the data.
    // - There's no mechanism being used to synchronize access to the data.

    // Data races cause undefined behavior and can be difficult to diagnose and fix when you're
    // trying to track them down at runtime; Rust prevent this problem by refusing to compile code
    // with data races!

    // As always, we can use curly bracketts to create a new scope, allowing for multiple mutable
    // references, just not simultaneous ones;

    // let mut s = String::from("hello");
    // {
    //     let r1 = &mut s;
    // } // r1 goes out of scope here, so we can make a new reference with no problems.
    // let r2 = &mut s;

    // Rust enforces a similar rule for combining a muttable and immutable references. This code
    // results in an error:

    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s;
    // println!("{}, {}, and {}", r1, r2, r3);

    // Whew! We also cannot have a mutable references while we have an immutable one to the same
    // value.

    // Uses of an immutable referene don't expect the value to suddenly change out from under them!
    // However, multiple immutable references are allowed because no one who is just reading the
    // data has the ability to affect anyone else's reading of the data.

    // Note that a reference's scope starts from where it is introduced and continues through the
    // last time that reference is used. For instance, this code will compoile because the last
    // usage of the immutable referenes, the println!, occurs before the mutable reference is
    // introduced:
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    //  variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    r3.push_str(", world");
    println!("{}", r3);
}

#[allow(dead_code)]
fn dangling_referenes() {
    // In languages with pointers, it's easy to erroneously create a dangling pointer - a
    // pointer thatt references a location in memory that may have been given to someonle else --
    // by freeing some memory while preserving a pointer to that memory. In Rust, by contrast, the
    // compiler guarantees that references will never be dangling references: if you have a
    // reference to some data, the compiler will ensure that the data will not go out of scope
    // before the reference to the data does.

    // Let's try to create a dangling reference to see how Rust prevents them with a compile-time
    // error:

    // let reference_to_nothing = dangle();

    // fn dangle() -> &String {
    //     // dangle returns a reference to a String
    //     let s = String::from("hello"); // s is a new String
    //     &s // we return a reference to the String, s
    // } // Here, s goes out of scope, and is dropped, Its memory goes away. // Danger!

    // The solutin here is to return the String directly:
    fn no_dangle() -> String {
        let s = String::from("hello");
        s
    }
    // this works without any problems. Ownership is moved out, and nothing is deallocated.
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn tedious_calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

// s is reference to a String
fn calculate_length(s: &String) -> usize {
    s.len()
} // Hre, s goes out of scope. But because it does not have ownership of what it refers to, it is
  // not dropped.

#[allow(unused_variables, dead_code, unused_mut)]
fn the_slice_type() {
    // The Slice Type

    // Slices let you reference a contiguous sequence of elements in a collection rather than the
    // whole collection. A slice is a kind of reference, so it doest not have ownership.

    // Here's a small programming problem: write a function that takes a string of words separated
    // by spaces and returns the first word it finds in that string. If the function doesn't find a
    // space in the string, the whole string mustt be one word, so the entire strings should be
    // returned.

    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is not totally invalid!

    // String Slices
    // A string slice is a reference to part of a String, and it looks like this:

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    // Rather than a reference to the entire String, hello is a reference to a portion of the
    // String, specified in the extra [0..5] bit. We create slice using a range within brackets by
    // specifying [starting_index..ending_index], where starting_index is the first position in the
    // slice and ending_index is one more than the last positio in the slice. Internally, the slice
    // data structure stores the starting position and the length of the slice, which corresponds
    // to ending_index minus starting_index. So, in the case of let world = &s[6..11];, world would
    // be a slice that contains a pointer to the byte at index 6 of s witth length value of 5.

    // With Rust's .. range syntax, if you want to start at index 0, you can drop the value before
    // the two periods. In other words, these are equal:
    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];

    // By the same token, if your slice includes the last byte of the String, you can drop the
    // trailing number. That means these are equal:
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];
    let slice = &s[3..];

    // You can also drop both values to take a slice of the entire string. So these are equal:
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];

    // The type that signifies "string slice" is written as &str;

    // We now have a straightforward API that's much harder to mess up because the compiler will
    // ensure the references into the String remain valid. Remember the bug in the program. When we
    // got the index to the end of the first word but then cleared the string so our index was
    // invalid? That code was logically incorrect but didn't show any immediate errors. the
    // problems would show up later if we kept trying to use the first tword index with an emptied
    // string. Slices make this bug impossible and let us know we have a problem with our code much
    // sooner. Using the slice version of first_word will throw a compile-time error:
    let mut s = String::from("raven paragas");
    let word = first_word(&s);
    // s.clear();
    println!("The first word is: {}", word);

    // String Literals as Slices
    // Recall that we talked about string literals being stored inside the binary. Now that we know
    // about slices, we can properly understand string literals:
    let s = "Hello, world!";
    // Tye type of s here is &str : it's a slice pointing to that specific point of the binary.
    // This is also why string literals are immutable; &str is an immutable reference.

    // String Slices as Parameters
    // Knowing that you can take slices of literals and String values leads us to one more
    // improvement on first_word, and that's its signature:

    // fn first_word(s: &String) -> &str {}

    // A more experienced Rustacean would write the signature shown instead because it allows us to
    // use the same function on both &String values and &str values.

    // fn first_word(s: &str) -> &str {}

    // If we have a string slice, we can pass that directly. If we have a String, we can pass slice
    // of the String or a reference to the String. This flexibility takes advantage of deref
    // coercions.

    // Defining a function to take a string slice instead of a reference to a String makes our API
    // more general and useful without losing any functionality:

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, wether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent to whole slices of
    // `String`s
    let word = first_word(&my_string);
    println!("{}", word);

    let my_string_literal = "raven paragas";
    // `first_word` works on slices of string literals, wether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{}", word);

    // Other Slices
    // String slices, as you might imagine, are specific to strings. But there's a more general
    // slice type too. Consider this array:
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    // This slice has the type &[i32]. It works the same way as string slices do, by storing a
    // reference to the first element and a length. You'll use this kind of slice for all sorts of
    // other collections.
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     &s[..]
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }
