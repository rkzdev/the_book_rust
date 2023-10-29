use traits::{notify, notify5, NewsArticle, Summary, Tweet};

fn main() {
    // Traits: Defining shared behavior
    // A trait defines functionality a particular type has and can share with other
    // types. We can use traits to define shared behavior in an abstract way. We can
    // use trait bounds to specify that a generic type can be any type that has
    // certain behavior.

    defining_trait();

    trait_as_parameters();
}

fn trait_as_parameters() {
    let article = NewsArticle {
        headline: "Headline".to_string(),
        content: "Content".to_string(),
        author: "Author".to_string(),
        location: "Location".to_string(),
    };

    notify(&article);
}

fn defining_trait() {
    // A type's behavior consist of the methods we can call on that type. Different
    // types share the same behavior if we can call the same methods on all of those
    // types. Trait definitions are a way to group method signatures together to
    // define a set of behavios necessary to accomplish some purpose.
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already known, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    println!("{}", tweet.with_default());
    notify5(&tweet);
}
