use std::io;

#[allow(unused_variables)]
fn main() {
    // variables_and_mutability();
    // constants();
    // shadowing();
    // scalar_types();
    // compound_types();
    functions();
}

#[allow(dead_code)]
fn variables_and_mutability() {
    // by default variables are immutable
    // let x = 5;

    // declare with mut if you want mutable variable to change value later
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

#[allow(dead_code)]
fn constants() {
    // limke immutable variables, constant are values that are bound to a name
    // and are not allowed change, but there are few differences between
    // constants and variables.

    // - mut are not allowed with constants. constants are always immutable
    // - you declare constant using const keyword and the type of the value must be annotated.
    // - constants can be declared in any scope, including global scope, which makes them useful for
    // values that many parts of code need to know about
    // - constants are valid for the entire time a program runs, within the scope they were
    // declared.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");
}

#[allow(dead_code)]
fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

#[allow(dead_code)]
fn scalar_types() {
    // Every value in rust is of a certain data type. We'll look at the two
    // data type subsets: scalar and compound.
    // Rust is a statically typed language.

    // let guess: u32 = "42".parse().expect("Not a number")
    let guess: u32 = match "42h".parse() {
        Ok(value) => value,
        Err(_) => 0,
    };
    println!("guess is: {guess}");

    // Scalar Types
    // integer, floats, numbers, booleans and characters.

    // 8-bit i8 u8
    // 16-bit i16 u16
    // 32-bit i32 u32
    // 64-bit i64 u64
    // 128-bit i128 u128
    // arch isize usize

    // to explicitly handle the possibility of overflow, you can use these families of methods
    // provided by the standard library for primitive numeric types.
    // - Wrap in all modes with the wrapping_* methods, such as wrapping_add.
    // - Return the None value if there is overflow with the checked_* methods.
    // - Return the value and boolean indicating whether there was overflow with the overflowing_* methods.
    // - Saturate at the value's minimum or maximum values with the saturating_* methods.

    // Floating-Point Types
    // f32 and f64

    // Numeric Operations

    let sum = 5 + 10;
    println!("sum: {sum}");
    let difference = 95.5 - 4.3;
    println!("difference: {difference}");
    let product = 4 * 30;
    println!("product: {product}");
    let quotient = 56.7 / 32.2;
    println!("quotient: {quotient}");
    let truncated = -5 / 3; // results in -1
    println!("truncated: {truncated}");
    let remainder = 43 % 5;
    println!("remainder: {remainder}");

    // The Boolean Type
    let t = true;
    println!("The value of t is: {t}");
    let f: bool = false; // with explicit type annotation
    println!("The value of f is: {f}");

    // The Character Type
    let c = 'z';
    let z: char = 'Z';
    let heart_yeard_cat = '😻';
    println!("{c} {z} {heart_yeard_cat}");
}

#[allow(dead_code, unused_variables)]
fn compound_types() {
    // Compound types can group multiple values into one type. Rust has two primitive
    // compound types: tuples and arrays.

    // The Tuple Type
    // A tuple is a general way of grouping together a number of values with variety of types
    // into one compound type. Tuples have a fixed length; once declared, they cannot grow
    // or shrink in size.

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);

    // The fvariable tup binds to the entired tuple because tuple is considered a single
    // compound element. To get the individual value of a tuple, we can use pattern matching
    // to destructure a tuple value, like this:
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // We can also access a tuple element directly using a period (.) followed by the index
    // we want to access. For example:
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{five_hundred} {six_point_four} {one}");

    // The tuple without any values has a special name, unit. This value and it's corresponding
    // type are both written () and represent and empty value or an empty return type. Expressions
    // implicitly return the unit value if they don't return any other value.

    // The Array Type
    // Another way to have a collection of multiple values is with an array. Unlike a tuple, every
    // element of an array must have the same type. Unlike arrays in some other languages, arrays
    // in Rust have a fixed length.

    let first_array = [1, 2, 3, 4, 5];
    println!("{:?}", first_array);

    // Arrays are useful when you want your data allocated on the stack rather than the heap or
    // when you want to ensure you always have fixed number of elements. An array isn't as flexible
    // as the vector type, though. A vector is a similar collection type provided by the standard
    // library that is allowed to grow or shrink in size. If you're unsure whether to use an array
    // or vector, chances are you should use a vector.

    // However, arrays are more useful when you know the number of elements will not need to
    // change.

    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    println!("{:?}", months);

    // Here, i32 is the type of each element. After the semicolon the number 5 indicates the array
    // contains five elements.
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    // You can also initialize and array to contain the same value for each element by specifying
    // the initial value, follwed by a semicolon, and the length of the array in square brackets.
    let a = [3; 5];
    println!("{:?}", a);

    // Accessing Array Elements
    // An array is a single chunk of memory of a known, fixed size that can be allocated on the
    // stack. You can access elements  of an array using indexing.

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    // Invalid Array Element Access

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");

    // If the input index is past the end of the array
    // The program resulted in a runtime error at the point of using an invalid value in the
    // indexing operation. The program exited with an error message and didn't execute the final
    // println statement. When you attempt to access an element using indexing. Rust will check
    // that the index you've specified is less than the array length. If the index is greater than
    // or equal to the legnth, Rust will panic. This check has to happen at runtime, especially in
    // this case, because the compiler can't possibly know what value a user will enter when they
    // run the code later.

    // This is an example of Rust's memory safety principles in action. In many low-level
    // languages, this kind of check is not done, and when you provide an incorrect index, invalid
    // memory can be accessed. Rust protects you againts this kind of error by immediately exiting
    // instead of allowing the memory access and continuing.
}

#[allow(dead_code, unused_variables)]
fn functions() {
    // In function signatures, you must declare the type of each parameter.
    fn another_function(x: i32) {
        println!("The value of x is: {x}");
    }

    another_function(5);

    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {value}{unit_label}");
    }

    print_labeled_measurement(5, 'h');

    // Statements and Expressions
    // - Statements are instructions that perform some action and do not return a value
    // - Expressions evaluate to resultant value.
    let y = 6;

    // let y = 6; is a statement

    // Statements do not return values. Therfore, you can't assign a let statement to another
    // variable
    // let x = (let y = 6);

    // Expressions evaluate to a value and make up most of the rest of that you'll write in Rust.
    // in the statement let y = 6; the 6 is an expression that evaluates to the value 6.
    // calling a function is an expression.
    // calling a micro is an expression.
    // a new scope block created with curly brackets is an expression.
    let y = {
        // this expression
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    // Expressions do not include ending semicolons. If you add a semicolon to the end of an
    // expression, you turn it into a statement, and it will then not return a value.

    // Functions with return values
    fn five() -> i32 {
        5
    }
    let x = five();
    println!("The value of x is: {x}");

    // fn plus_one(x: i32) -> i32 {
    //     x + 1;
    // }

    // Control Flow

    // if Expressions
    let number = 3;
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    println!("1900 is leap year: {}", is_leap_year(1900));
    println!("2000 is leap year: {}", is_leap_year(2000));
    println!("1904 is leap year: {}", is_leap_year(1904));

    println!("1 is odd: {}", is_odd(1));
    println!("2 is odd: {}", is_odd(2));

    // Repetition with Loops
    // The loop keyword tells Rust to execute a block of code over and over again
    // forever or until you explicitly tell it to stop.

    // infinite loop
    // loop {
    //     println!("again!");
    // }

    // Returning values from loops
    {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {result}");
    }

    // Loop labels to disambiguate between multiple loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaing = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // Conditional Loops with while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // let result = selection_search(&mut vec![5, 4, 3, 2, 1]);
    // println!("{:?}", result);

    // match binary_search(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5) {
    //     Some(x) => println!("Result is: {x}"),
    //     None => println!("Item not found!"),
    // }

    // Looping Through a Collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < a.len() {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    println!("100 farhenheit is : {}", farhenheit_to_celius(100.0));

    println!("fib of 10 is: {}", fib(10));

    twelve_day_of_christmas();
}

fn is_leap_year(year: u16) -> bool {
    let mut is_leap_year = false;

    if year % 4 == 0 && (year % 400 == 0 || year % 100 != 0) {
        is_leap_year = true;
    }

    is_leap_year
}

fn is_odd(number: i32) -> bool {
    // if number % 2 == 0 {
    //     return false;
    // }
    // return true;
    return if number % 2 == 0 { false } else { true };
}

#[allow(dead_code, unused_variables)]
fn find_smallest(numbers: &mut Vec<i32>) -> usize {
    let mut smallest = numbers[0];
    let mut smallest_index: usize = 0;
    let mut count = 0;

    loop {
        if count < numbers.len() {
            if smallest > numbers[count] {
                smallest = numbers[count];
                smallest_index = count;
            }
            count += 1;
        } else {
            break;
        }
    }

    smallest_index
}

#[allow(dead_code, unused_variables)]
fn selection_search(numbers: &mut Vec<i32>) -> Vec<i32> {
    let mut sorted: Vec<i32> = vec![];
    let length = numbers.len();
    let mut count = 0;

    loop {
        if count < length {
            let smallest = find_smallest(numbers);
            sorted.push(numbers.remove(smallest));
            count += 1;
        } else {
            break;
        }
    }

    return sorted;
}

fn binary_search(numbers: &[i32], number: i32) -> Option<usize> {
    let mut low: usize = 0;
    let mut high: usize = numbers.len() - 1; // 5

    while low <= high {
        println!("low: {low}, high: {high}");
        let middle = (low + high) / 2;
        let guess = numbers[middle as usize];

        println!("guess is : {guess}");

        if guess == number {
            return Some(middle - 1);
        }

        if guess > number {
            high = middle - 1;
        } else {
            low = middle + 1;
        }
    }

    return None;
}

fn farhenheit_to_celius(farhenheit: f64) -> f64 {
    (farhenheit - 32.0) * 5.0 / 9.0
}

fn fib(number: i32) -> i32 {
    if number == 1 || number == 2 {
        return 1;
    }

    return fib(number - 1) + fib(number - 2);
}

fn twelve_day_of_christmas() {
    let days = [
        "twelve", "eleven", "ten", "nine", "eight", "seventh", "six", "five", "four", "three",
        "two", "one",
    ];

    for day in days {
        println!("In the {day} for christmas, my true love sent to me");
    }
}
