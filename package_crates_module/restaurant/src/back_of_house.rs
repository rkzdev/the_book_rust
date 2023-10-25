// If we make an enum public, all of its variants are public.
pub enum Appetizer {
    Soup,
    Salad,
}

// Structs are often useful without their fields being public, so struct fields
// follow the general rule of everything being private by default unless annotated
// with pub.
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

fn fix_incorrect_order() {
    cook_order();
    super::deliver_order();
    // crate::deliver_order();
}

fn cook_order() {}
