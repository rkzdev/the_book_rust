use std::{
    cmp::Ordering,
    error::Error,
    fs::{self, File},
    io::{self, ErrorKind, Read},
    net::IpAddr,
};

use rand::Rng;

// So far, all the main functions we've used return (). The main function is
// special because it's the entry and exit point of executable programs, and
// there are restrictions on what its return type can be for the programs to
// behave as expected.

// Luckily, main can also return a Result<(), E>. but we've changed the return
// type of main to be Result<(), Box<dyn Error>> and added a return value Ok(())
// to the end. This code will now compile

// The Box<dyn Error> type is a trait object. For now, you can read Box<dyn Error>
// to mean "any kind of error". Using ? on a Result value in a main function with
// the error type Box<dyn Error> is allowed, because it allows any Err value to be
// returned early. Even though the body of this main function will only ever return
// errors of type std::io::Error, by specifying Box<dyn Error>, this signature
// will continue to be correct even if more code that returns other errors is added
// to the body of main.

// When a main function returns a Result<(), E>, the executable will exit with a value
// of 0 if main returns Ok(()) and will exit with a nonzero value if main returns an
// Err value. Executables written in C return integers when they exit: programs that
//exitt successfully return the integer 0, and programs that error return some integer
// other than 0. Rust also returns integers from executables to be compatible with
// this convention.

// The main function may return any types that implement the std::process::Termination
// trait which contains a function report that return an ExitCode.
fn main() -> Result<(), Box<dyn Error>> {
    // Rust groups errors into two major categories: recoverable and unrecoverable errors.
    // For a recoverable error, such as file not found error, we most likely just want
    // to report problem to the user and retry the operation. Unrecoverable errors are
    // always symptoms of bugs, like trying to access a location beyound the end of
    // an array, and so we want to immediately stop the program.

    // Most languages don't distinguish between these two kinds of errors and handle both
    // in the the same way, using mechanisms such as exceptions. Rust doesn't have exceptions.
    // Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that
    // stops execution when the program encounters an unrecoverable error.
    // unrecoverable_errors_with_panic();

    // recoverable_errors_with_result();

    // propagating_errors();

    // panic_or_not_to_panic();

    creating_custom_types_for_validation();

    // let greeting_file = File::open("hello.txt")?;
    Ok(())
}

#[allow(unused_variables, dead_code)]
fn creating_custom_types_for_validation() {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }

    let secret_number = rand::thread_rng().gen_range(1..=10);

    loop {
        let mut guess = String::new();

        println!("Enter a number: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid number");
                continue;
            }
        };

        let guess = Guess::new(guess);

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("too low"),
            Ordering::Greater => println!("too high"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}

#[allow(unused_variables, dead_code)]
fn panic_or_not_to_panic() {
    // Returning result is a good default choise when you're defining a function
    // that might fail.

    // In situations such as examples, prototype code, and tests, it's more appropriate
    // to write code that panics instead of returning a Result.

    // Examples, Prototype Code, and Tests

    // In examples, it's understood that a call to a method like unwrap that could panic
    // is meant as a placeholder for the way you'd want your application to handle errors,
    // which can differ based on what the rest of your code is doing.

    // Similarly, the unwrap and expect methods are very handy when prototyping, before
    // you're ready to decided how to handle errors. They leave clear markers in your code
    // for when you're ready to make your program more robust.

    // If a method call fails in a test, you'd want the whole test to fail, event if that
    // method isn't the functionality under test. Because pani! is how a test is marked
    // as a failure, calling unwrap or expect is exactly what should happen.

    // Cases in which you have more information than the compiler
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    // We're creating an IpAddr instance by parsing a hardcoed string. We can see that
    // 127.0.0.1 is a valid IP address, so it's acceptable to use expect here. However,
    // having a hardcoded, valid string doesn't change the return type of the parse method
    // we still get a Result value, and the compiler will still make us handle the Result
    // as if the Err variant is a possibility because the compiler isn't smart enough to
    // see that this string is always a valid IP address. If the IP address string came
    // from a user rather than being hardcoded into the program and therefore did have a
    // possibility of failure, we'd definitely want to handle the Result in a more robust
    // way instead. Mentioning the assumption that this IP address is hardcoded will prompt
    // us to change expect to better error handling code if in the future, we need to get
    // the IP address fro some other source instead.
}

#[allow(unused_variables, dead_code)]
fn propagating_errors() {
    // When a function's implementation calls something that might fail, instead of handling
    // the error within the function itself, you can return the error to the calling code
    // so that it can decide what to do. This is known as propagating the error and gives more
    // control to the calling code, where there might be more information or logic that
    // dictates how the error should be handled than what you have available in the context
    // of your code

    // This function can be written in a much shorter way, but we're going to start
    // by doing a lot of it manually in order to explore error handling; at the end,
    // we'll show the shorter way. Let's look at the return type of the function first:
    // Result<String, io::Error>. This means the function is returning a value of the
    // type Result<T, E> where the generic parameter T has been filled in with the
    // concrete type String, and the generic type E has been filled in with the concrete
    // type io::Error.

    // If this function succeeds without any problems, the code that calls this function
    // will receive an Ok value that holds a String --the username that this function
    // read from the file. If this function encounters any problems, the calling code
    // will receive an Err value that holds an instance of io::Error that contains more
    // information about what the problems were. We chose io::Error as the return type
    // of this function because that happens to be the type of the error value returned
    // from both the operations we're calling in this function's body that might fail:
    // the File::open function and the read_to_string method.

    // The body of the function starts by calling the File::open function. Then we handle
    // Result value with a match. If File::open succeeds, the file handle in the pattern
    // variable file becomes the value in the mutable variable username_file and the
    // function continues. In the Err case, instead of calling panic!, we use the return
    // keyword to return early out of the function entirely and pass the error value from
    // File::open, now in the pattern variable e, back to the calling code as this function's
    // error value.
    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // Propagating errors is so common in Rust that Rust provides the question
    // mark operator ? to make it easier.

    // The ? placed after a Result value is defined to work in almost the same way
    // as the match expressions we defined to handle the Result values. If the value
    // of the Result is an Ok, the value inside the Ok will get returned from this
    // expression, and the program will continue. If the value is an Err, the Err will
    // be returned from the whole function as if we had used the return keyword so the
    // error value gets propagated to the calling code.

    // There is a difference between what the match expression and what the ? operator
    // does: error values that have the ? operator called on them go through the from
    // function, defined in the From trait in the standard library, which is used to
    // convert values from one type into another. When the ? operator calls the from
    // function, the error type received is converted into the error typed defined in
    // the return type of the current function. This is useful when a function returns
    // one error type to represent all the ways a function might fail, even if parts
    // might fail for many different reasons.
    fn read_username_from_file_2() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    // The ? operator eliminates a lot of boilerplate and makes this function's
    // implementation simpler. We could even shorten this code further by chaining
    // method calls immediately after the ?
    fn read_username_from_file_3() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    // way to make this even shorter using fs::read_to_string.
    fn read_username_from_file_4() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    // Where the ? operator can be used
    // The ? operator can only be used in functions whose return type is compatible
    // with the value the ? is used on. This is because the ? operator is defined
    // to performed an early return of a value of the function, in the same manner
    // as the match expression. The match was using a Result value, and the early
    // return arm returned an Err(e) value. The return type of the function has to
    // be a Resultt so tthat it's compatible with this return.

    // This code opens a file, which might fail. The ? operator follows the Result
    // value returned by File::open, but this main function has the return type of (),
    // not Result. When we compile this code, we get the following error:

    // fn test_me() {
    //     let greeting_file = File::open("hello.txt")?;
    // }

    // This error points out that we're only allowed to use the ? operator in a function
    // that returns Result, Option, or another type that implements FromResidual.

    // To fix the error, you have two choices. Once choise is to change the return type
    // of your function to be compatible with the value you're using the ? operator on
    // as long as you have no restrictions preventing that. The other technique is to
    // use a match or one of the Result<T, E> methods to handle the Result<T, E> in
    // whatever way is appropriate.

    // The error message also mentioned that ? can be used with Option<T> values as well.
    // As with using ? on Result, you can only use ? on Option in a function that returns
    // an Option. The behavior of the ? operator when called on an Option<T> is similar
    // to its behavior when called on a Result<T, E>: if the value is None, the None will
    // be returned early from the function at that point. If the value is Some, the value
    // inside the Some is the resulting value of the expression and the function continues,
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }

    match last_char_of_first_line("") {
        Some(value) => println!("{value}"),
        None => println!("empty string is passed"),
    }

    // Note that you can't mix and match. The operator won't automatically convert
    // a Result to an Option or vice versa; in those cases, you can use methods like
    // the ok method on Result or the ok_or method on Option to do the conversion
    // explicitly.

    let result = read_username_from_file().expect("no username found");

    println!("{result}");
}

#[allow(unused_variables, dead_code)]
fn recoverable_errors_with_result() {
    // Most erros aren't serious enough to require the program to stop entirely. Sometimes,
    // when a function fails, it's for a reason that you can easily interpret and respond to.
    // For example if you try to open a file and that operation fails because the file doesn't
    // exist, you might want to create the file instead of terminating the process.

    // Result enum is defined as having two variants, Ok and Err, as follows:
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }
    // The T and E are generic types parameters: What you need to know right now is that T
    // represent the type of the value that will be returned in a success case within the Ok
    // varaint, and E represents the type of the error that will be returned in a failure case
    // within the Err variant. Because Result has these generic type parameters, we can use
    // the Result type and the functions defined on it in many different situations where
    // the successful value and error value we want to return may differ.

    // let greeting_file_result = File::open("hello.txt");

    // The return type of File::open is a Result<T, E>. The generic parameter T has been
    // filled in by the implementation of File::open with the type of the success value,
    // std::ds::File, which is a file handle. The type of E used in the error value is
    // std::io::Error. This return type means the call to File::open might success and
    // return a file handle that we can read from or write to. The function call also
    // might fail: for example, the file might not exists, or we might not have permission
    // to access the file. The File::open function needs to have a way to tell us whether
    // it succeeded or failed and at the same time give us either the file handle or
    // error information. This information is exactly what the Result enum conveys.

    // In the case where File::open succeeds, the value in the variable greeting_file_result
    // will be an instance of Ok that contains a file handle. In the case where it fails, the
    // value in greeting_file_result will be an instance of Err that contains more information
    // about the kind of error that happened.

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // Note that, like the Option enum, the Rsult enum and its variants have been brought
    // into scope by the prelude, so we don't need to specify Result:: before the Ok and Err
    // varaints in the match arms.

    // When the rsult is Ok, this code will return the inner file value out of the Ok variant,
    // and we then assign the file handle value to the variable greeting_file. After the match,
    // we can use the file handle for reading or writing.

    // Matching on Different Errors
    // The code will panic! no matter why File::open failed. However, we want to take
    // different actitos for different failure reasons: If File::open failed because the
    // file doesn't exist, we want to create the file and return the handle to the new file.
    // If File::open failed for any other reason -- for example, because we didn't have
    // permissio to open the file -- we still want the code to panic! in the same way as
    // it did previously

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Alternative to Using math with Result<T, E>
    // That's a low of match! The match expression is very useful but also very much
    // a primitive. In Chapter 13, you'll learn about closures, which are used with
    // many of the methods defined on Result<T, E>. These methods can be more concise
    // than using mtach when handling Result<T, E> values in your code.

    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Probleam creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Althoug this code has the same behavior, it doesn't contain any match expressions
    // and is cleaner to read. Come back to this example after you've read chapter 13, and
    // look up the unwrap_or_else method in the standard library documentation. Many
    // more of these methods can clean up huge nested match expressions when you're
    // dealing with errors.

    // Shortcuts for Panic on Error: unwrap and expect
    // Using match works well enough, but i can be a bit verbose and doesn't always
    // communicate intent well. The Result<T, E> type has many helper methods defined
    // on it to do various, more specific tasks. The unwrap method is a shortcut method
    // implemented just like the match expression we wrote. If the Result value is Ok
    // variant, unwrap will return the value inside the Ok. If the Result is the Err
    // varaint, unwrap will call the panic! macro for us.

    let greeting_file = File::open("hello.txt").unwrap();

    // Similarly, the expect method lets us also choose the panic! error message. Using
    // expect instead of unwrap and providing good error messages can conver your intent
    // and make tracking down thte source of a panic easier.

    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // We use expect in the same ways as unwrap : to return the file handle or call the
    // panic! macro. The error message used by expect in its call to panic! will be the
    // parameter that we pass to expect, rather than the default panic! message that
    // unwrap uses.

    // In productin-quality code, most Rustaceans choose expect rather than unwrap and
    // give more context about why the operation is expected to always succeed. That way,
    // if your assumptions are ever proven wrong, you have more information to use in
    // debugging.
}

#[allow(dead_code)]
fn unrecoverable_errors_with_panic() {
    // There are two ways to cause a panic in practice: by taking an action that causes
    // our code to panic (such as accessin an array past the end) or by explicitly
    // calling the panic! macro. By default, these panics will print a failure message
    // unwind, clean up the stack, and quit. Via an environment variable, you can
    // also have Rust dipslay the call stack when a panic occurs to make it easier
    // to track down the source of the panic.

    // Unwinding the Stack or Aborting in Response to a Panic
    // Be default, when panic occurs, the program starts unwinding, which means Rust walks
    // back up the stack and cleans up the data from each function it encounters. However,
    // this walking back and cleanup is a lot of work. Rust, therefor, allows you to choose
    // the alternative of immediately abortting, which ends the program without cleaning up.

    // Memory that the program was using will then need to be cleaned up by the operating
    // system. If in your project you need to make the resulting binary as small as possible,
    // you can switch from unwinding to aborting upon a panic by adding panic = 'abourt' to
    // the appropriate [profile] sections in your Cargom.toml file. For example, if you want
    // to abort on panic in release mode, add this:
    // [profile.release]
    // panic = 'abort'

    // panic!("crash and burn");

    // Using a panic! Backtrace
    // Let's look to another example to see what it's like when a panic! call comes
    // from a library because of a bug in our code instead of from our code calling the
    // macro directly.

    let v = vec![1, 2, 3];

    v[99];

    // The error point where we attempt to access index 99. The next node line tells us
    // that we can set the RUST_BACKTRACE environment variable to get a backtrace of
    // exactly what happened to cause the error. A backtrace is a list of all the functions
    // that have been called to get to this point. Backtraces in rust work as tehy do in
    // other languages: the key to reading the backtrace is to start from the top and read
    // until yo see files you wrote. That's the spot where the problem orignated. The lines
    // above the spot are code that your code has called; the lines below are code that
    // called your code. These before-and-after lines might include core Rust code,
    // standard library code, or crates that you're using.

    // In order to get backtraces with this information, debug symbols must be enabled.
    // Debug symbols are enabled by default when using cargo build or cargo run without
    // the --release flag.

    // We will come back to panic! and when we should and should not use panic! to handle
    // error conditions.
}
