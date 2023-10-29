use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String;

    fn with_default(&self) -> String {
        String::from("read more...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "reply: {}, retweet: {}", self.reply, self.retweet)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
// The impl Trait syntax works for straightforward cases but is actually syntax
// sugar for a longer form known as a trait bound;
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Using impl Trait is appropriate if wen this function to allow item1 and item2
// to have different type (as long as both types implement Summary).
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {}

// If we want to force both parameters to have the same type, however, we must
// use a trait bound, like this:
pub fn notify4<T: Summary>(item1: &T, item2: &T) {}

// Specifying Multiple Trait Bounds with the + Syntax
// we specify in the notify5 definition that item must implement both Display and
// Summary. We can do so using the + syntax
pub fn notify5(item: &(impl Summary + Display)) {
    println!("{}", &item);
}

// The + syntax is also valid with trait bounds on generic types:
pub fn notify6<T: Summary + Display>(item: &T) {}

// Clearer Trait Bounds with where Clauses
// Using too many trait bounds has its downsides. Each generic has its own trait bounds,
// so functions with multiple generic type parameters can contain lots of trait bound
// information between the function's name and its parameter list, making the function
// signature hard to read. For this reason, Rust has alternative syntax for specifying
// trait bounds inside a where clause after the function signature. So intead of
// writing this:
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// we can use a where caluse, like this:
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    8
}

// Returning Types that Implement Traits
// By using impl Summary for the return type, we specify that the returns_summarizable
// function returns some type that implements the Summary trait without naming the
// concrete type. In this case, returns_summarizable returns a Tweet, but the code
// calling this functions doesn't need to know that.

// the ability to specify a return type only by the trait it implements is especially useful
// in the context of closures and iterators. Closures and iterators create type that
// only the compiler knows or types that are very long to specify. Tme impl Trait
// syntax lets you concisely specify that a function return some type that implements
// the Iterator trait without needing to write out a very long type.

// However, you can only use impl Trait if you're returning a single type.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: "horse_ebook".to_string(),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// Using Trait Bounds to Conditionally Implement Methods
// By using a trait bound with an impl block that uses generic type parameters,
// we can implement methods conditionally for types that implement the specified
// traits.
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmd_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// We can also conditionally implement a trait for any type that implements another
// trait. Implementations of a trait on any type that satisfies the trait bounds are
// called blanket implementations and are extensively used in the Rust standard library.
// For example, the standard library implements the ToString trait on any type that
// implements the Display trait. The impl block in the standard library looks
// similar to this code:
// impl<T: Display> ToString for T {}
// Because the standard library has this blanket implementation, we can call the
// to_string method defined by the ToString trait on any type that implements the
// Display trait, we can turn integers into their corresponding String values like
// this because integers implement Display:
// let s = 3.to_string();
