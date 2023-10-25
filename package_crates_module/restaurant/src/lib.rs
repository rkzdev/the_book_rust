// this is the parent module
// parent module can;t use the private items inside child modules

// this is a child module
// child modules can use the items in their parent modules. because child module
// can see the context in which they are defined.
mod front_of_house;

// Starting relative paths with super
// We can construct relative paths that begin in the parent module, rather than
// the current module or the crate root, by using super at the start of the path.
// This is like starting a filesystem path with .. syntax. Using super allows us
// to reference an item that we know is in the parent module, which can make rearranging
// the module tree easier when the module is closely related to the parent, but
// the parent might be moved elsewhere in the module tree someday.
mod back_of_house;

// Bringing Paths into Scope with the use Keyword
// Having to write out the paths to call functions can feel inconvenient and repetitive.
// Whether we chose the absolute or relative path to the add_to_waitlist() function, every
// time we wanter to call add_to_waitlist we had to specify front_of_house and hosting too.
// Fortunately, there's a way to simplify this process: we can create a shortcut to a path
// with the use keyword once, and then use the shorter name everywhere else in the scope.

// Adding use and a path in a scope is similar to creating a symbolic link in the filesystem.
// By adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name
// in that scope, just as though the hosting module had been define in the crate root. Paths
// brought into scope with use also check privacy, like any other paths.
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// mod customer {
//     pub fn eat_at_restaurant() {
//         hosting::add_to_waitlist();
//     }
// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();

//     let mut meal = back_of_house::Breakfast::summer("Rye");
//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
// }

fn deliver_order() {}
