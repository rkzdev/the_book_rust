use std::thread;

// Closures: Anonymous Functions that Capture Their Environment

// Rust's closures are anonymous functions you can save in a variable or pass as
// arguemnts to other functions. You can create the closure in one place and then
// call the closure elsewhere to evaluate it in a different context. Unline functions,
// closures can capture values from the scope in which they're defined.

// Capturing the Environment with Closures
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preferences: Option<ShirtColor>) -> ShirtColor {
        user_preferences.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Caputring References or Moving Ownership

    // Closures can capture values from their environment in three ways,
    // which directly map to the three ways a function can take a parameter:
    // borrowing immutably, borrowing mutably, and taking ownership. The closure
    // will decide which of these to use based on what the body of the function
    // does with the captured values.

    // Example: closure that captures an immutable reference to the vector named
    // list because it only needs an immutable reference to print the value:
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // Example: Closure that captures a mutable reference
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    // Example: Closure can take ownership via the move keword before the parameter
    // list. This technique is mostly useful when passing a closure to a new thread
    // to move the data so that it's owned by the new thread.
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // Moving Captured Values Out of Closures and the Fn Traits

    // Once a closure has captured a reference or captured ownership of a value from
    // the environment where the closure is defined, the code in the body of the closure
    // defines what happens to the fererences or value when the closure is evaluated later
    // A closure body can do any of the following: move a captured value out of the closure,
    // mutate the captured value, neither move or mutate the value, or capture nothing
    // from the environment to begin with.

    // The way a closure captures and handles values from the environment affects
    // which traits the closure implements, and traits are how functions and structs
    // can specify what kinds of closures they can use. Closures will automatically
    // implement one, two, or all three of these Fn traits, in an additive fashion,
    // depending on how the closure's body handles the values:

    // 1. FnOnce applies to closures that can be called once. All closures implement
    // at least this trait, because all closures can be called. A closure that moves
    // captured values out of its body will implement FnOnce and none of the other Fn
    // traits, because it can only be called once.

    // 2. FnMut applies to closures that don't move captures values out of their body,
    // but that might mutate the captured values. These closures can be called more
    // than once.

    // 3. Fn applies to closures than don't move captured values out of their body
    // and that don't mutate captured values, as well as closures that capture nothing
    // from their environment. These closures can be called more than once without
    // mutating their environment, which is important in cases such as calling a
    // closure multiple times concurrently.

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // The reason sort_by_key is defined to take an FnMut closure is that it calls
    // the closures multiple times: once for reach item in the slice. The closure
    // |r| r.width doesn't capture, mutate, or move out anything from its environment
    // so it meets the trait tbound requirements.
    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    // Error: using FnOnce
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        // sort_operations.push(value);
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}", list);

    // The Fn traits are important when defining or using functions or types
    // that make use of closures.
}
