fn main() {
    // We'll look at enumerations, also referred to as enums. Enums allow you to define a type by
    // enumerating its possible variants. First we'll defined and use an enum to show how an enum
    // can encode meaning along with data. Next, we'll explore a praticularly useful enum, called
    // Option, which expresses that a value can be either something or nothing. Then we'll look at
    // how pattern matching in the match expression makes it easy to run differentt code for
    // different values of an enum. Finally, we'll cover how the if let construct is another
    // convenient and concise idiom available to handle enums in your code.

    defining_an_enum();

    option_enum();

    match_control_flow();

    patterns_that_bind_to_values();

    matching_with_options();

    catch_all_and_the_placeholder();

    concise_control_flow_with_if_let();
}

fn concise_control_flow_with_if_let() {
    // The if let syntax lets you combine if and let into a less verbose way to handle values that
    // match one pattern while ignoring the rest. Consider the program that matches on an
    // Option<u8> value in the config_max variable but only want to execute code if the value is
    // the Some variant.

    // If the value is Some, we print out the value in the Some variant by binding the value to the
    // variable max in the pattern. We don't want to do anything with the None value. To satisfy
    // the match expression, we have to add _=> () after processing just one variant, which is
    // annoying boilerplate code to add.
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // Instead, we could write this is a shorter way using if let. The following code behaves the
    // same as the match:
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // Using if let means less typing, less indentation, and less boilerplate code. However, you
    // lose the exhaustive checking that match enforces.
    #[derive(Debug)]
    enum UsState {
        Alaska,
        Alabama,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarts(UsState),
    }

    // let mut count = 0;
    // let coin = Coin::Penny;

    // match coin {
    //     Coin::Quarts(state) => println!("State quarter from: {:?}", state),
    //     _ => count += 1,
    // }
    // or we could use an if let and else expression, like this:
    let mut count = 0;
    let coin = Coin::Quarts(UsState::Alaska);
    if let Coin::Quarts(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}

fn catch_all_and_the_placeholder() {
    // Using enums, we can also take special actions for a frew particular values, but for all
    // other values take one default action. Imaging we're implementing a game where, if you roll a
    // 3 on a dice roll, you player doesn't move, but instead get a new fancy hat. If you roll a 7,
    //   your player loses a fancy hat. For all other values, your player moves that number of
    //   spaces on the game board. Here's a match that implements that logic, with the result of
    //   the dice roll hardcoded rather than a random value,

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}

    // for the first two arms, the patterns are the literal values 3 and 7. For the last arm that
    // covers every other possible value, the pattern is the variable we've chosen to name other.
    // The code that runs for the other arm uses the variable by passing it to the move_player
    // function.

    // Rust also has a pattern we can use when we want a catch-all but don't want to use the value
    // in the catch-all pattern: _ is a special pattern that matches any value and does not bind to
    // that value. This tells rust we aren't going to use the value, so Rust won't warn us about an
    // unused variable.

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn reroll() {}

    // Finally, we'll change the rules of the game on more time so that nothing else happens on
    // your turn if you roll anything other than a 3 or a 7.
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

fn matching_with_options() {
    // Matches are Exhaustive

    // There's one other aspect of match we need to discuss: the arm's patterns must cover all
    // possiblities. Consider this version of our plus_one function, which has a bug and won't
    // compile.
    // We didn't thandle the None case, so this code will cause a bug. Luckily, it's a bug Rust
    // knows how to catch,
    // Rust knows that we didn't cover every possible case, and even knows which pattern we forgot!
    // matches in rust are exhaustive:
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    dbg!(six);
    dbg!(none);
}

fn patterns_that_bind_to_values() {
    // Another useful feature of match arms is that hey can bind to the parts of the values that
    // match the pattern. This is how we can extract values out of enum variants.

    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    let quart = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(quart));
}

// The match Control Flow Construct
fn match_control_flow() {
    // match allows you to compare a value againts a series of patterns and then execute code base
    // on which pattern matches. Patterns can be made up of literal values, variable names,
    // wildcards, and many other things;

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: &Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    let penny = Coin::Penny;
    let value_in_cent = value_in_cents(&penny);
    println!("{:#?} has a value of: {}", penny, value_in_cent);

    // We don't typically use curly brackets if the match arm code is hort. If you want to run
    // multiple lines of code in a match arm, you must use curly brackets, and the comma following
    // the arm is then optional.

    fn value_in_cents_2(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

fn defining_an_enum() {
    // Where structs give you a way of grouping together related fields and aata, enums give you a
    // way of saying a value is one of a possible set of values. For example, we may want to say
    // that Rectangle is one of a set of possible shapes that also includes Circle and Triangle.

    // Let's look at a situation we might want to express in code and see why enums are useful and
    // more appropriate than structs in this case. Say we need to work with IP addresses.
    // Currently, two major standards are used for IP addresses: version four and version six.
    // Because these are the only possibilities for an IP address that our program will come
    // across, we can enumerate all possible variants, which is where enumeration gets its name.

    // Any IP address can be either a version four or a version six address, but not both at the
    // same time. That property of IP addresses makes the enum data structure appropriate because
    // an enum value can only be one of itts variants. Both version four and version six addresses
    // are still fundamentally IP addresses, so they should be treated as the same type when the
    // code is handling situations that apply to any kind of IP address.

    // We can express this concept in code by defining an IpAddrKind enumeration and listing the
    // possible kinds an IP address can be, V4 and V6. These are the variants of the enum:
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }
    // // IpAddrKind is now a custom data type that we can use elsewhere in our code.
    // fn route(_ip_kind: IpAddrKind) {}
    // // Note that the variants of enum are namespaced under its identifier, and we use a double
    // // colon to separate the two. This is useful because now both values IpAddrKind::V4 and
    // // IpAddrKind::V6 are of the same type: IpAddrKind. We can then, for instance, define a
    // // function that takes any IpAddrKind:
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    // route(four);
    // route(six);

    // We attach data to each variant of the enum directly, so there is no need for an extra
    // strcut. Here, it's also easier to see another detail of how enums work: tthe name of each
    // enum variant that we define also becomes a function that constructs an instance of the enum.
    // That is, IpAddr::V4() is a function call that takes a String argumentt and returns an
    // instance of the IpAddr type. We automatically get this constructor function defined as a
    // result of defining the enum.
    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }
    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    // There's another advantage to using an enum rather than a struct: each variant can have
    // different types and amounts of associated data. Version four IP addresses will always have
    // four numeric components tthat will have a values between 0 and 255. If we wanter to store V4
    // addresses as four u8 values but still express V6 addresses as one String, we wouldn't be
    // able to with a struct. Enums handle this case wtih ease:

    // enum IpAddr {
    //     V4(u8, u8, u8, u8),
    //     V6(String),
    // }

    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));

    // Standard librarry has a definition we can use!
    // Let's look at how the standard library defines IpAddr:

    struct Ipv4Addr {
        //
    }

    struct Ipv6Addr {}

    // This code illustrates that you can put any kind of data inside an enum variant: strings,
    // numeric types, or structs. You can even include another enum!
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    // This enum has four variants with different types:
    // Quit has no data associated with it at all.
    // Move has named fields, like a struct does.
    // Write includes a singles String.
    // ChangeColor includes three i32 values.
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // Just as we'er able to define methods on structs using impl, we're also able to define
    // methods on enums.
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

// The Option Enum and Its Advantages Over Null Values
fn option_enum() {
    // Option is another enum defined by the standard library. The Option type encodes the very
    // common scenario in which a value could be something or it could be nothing.

    // For example, if you request the first item in a non-empty list, you would get a value. If
    // you request the first item in an empty list, you would get nothing. Expressing this concept
    // in terms of the type system means the compiler can check whether you've handled all the
    // cases you should be handling; this functionality can prevent bugs that are extremely common
    // in other programming languages.

    // Programming language desing is often thoughtt of in terms of which features you include, but
    // the features you exclude are important too. Rust doesn't have the null feature that many
    // other languages have. Null is a value that means tthere is no value there. In languages with
    // null, variable can always be in on of ttwo states; null or not-null.

    // In his 2009 presentation: "Null Referenxces: The Billion Dollar Mistake," Tony Hoare

    // The problem with null values is that if you try to use a null value as a not-null value,
    // you'll get an error of some kind. Because this null or not-null property is pervasive, it's
    // extremely easy to make this kind of error.

    // However, the concept that null is trying to express is still a useful one: a null is a value
    // that is currently invalid or absent for some reason.

    // The problem isn't really with the concept but with the particular implementation. As such,
    // Rust does not have nulls, but it does have an enum that can encode the concept of a value
    // being present or absent. This enum is Option<T>, and it is defined in the standard library.

    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    // The Option<T> enum is so useful that it's even included in the prelude; you don't need to
    // bring it into scope explicitly. Its variant are also included in the prelude: you can use
    // Some and None directly without Option:: prefix. The Option<T> enum is still just a regular
    // enum, and Some(T) and None are still variantts of tytpe Option<T>.

    // The <T> syntax is a feature of Rust we haven't talked about yet. It's a generic type
    // parameter. All you need to know is that <T> means that the Some variant of the Option enum
    // can hold one piece of data of any type, and that each concrete type that gets used in place
    // of T makes the overall Option<T> type a different type.

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // The type of some_number is Option<i32>. The type of some_char is Option<char>, which is
    // different type. Rust can infer these types because we've specified a value inside the Some
    // varaint. For absent_number, Rust requires us to annotate the overall Option type: the
    // compiler can't infer the type that the corresponding Some variant will hold by looking only
    // at a None value. Here, we tell Rust that we mean for absent_number to be of type Option<i32>

    // When we have a Some value, we know that a value is present and the value is held within the
    // Some. When we have a None value, in some sense it means the same thing as null: we don't
    // have a valid value. So why is having Option<T> any better than having null?

    // Because Option<T> and T (where T can be any type) are different types, the compiler won't
    // let us use an Option<T> value as if it were definitely a valid value. For example, this code
    // won't ocmpile, because it's trying to add an i8 to an Option<i8>:
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    // let sum = x + y;

    // Intense! in effect, this error message means that Rust doesn't understand how to add an i8
    // and an Option<i8>, because they're different types. When we have a value of a type like i8
    // in Rust, the compiler will ensure that we always have a valid value. We can proceed
    // confidently without having to check for null before using that value. Only when we have an
    // Option<i8> do we have to worry about possibly not having a value, and the compiler will make
    // sure we handle that case before using the value.

    // In other words, you have to convert an Option<T> to a T before you can perform T operations
    // with it. Generally, this helps catch one of the most common issues with null; assuming that
    // something isn't null when it actually is.

    // The match expression is a control flow construct that does just this when used with enums:
    // it will run different code depending on which variant of the enum it has, and that code can
    // use the data inside the matching value.
}
