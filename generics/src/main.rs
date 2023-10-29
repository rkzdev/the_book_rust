// Because we want to compare values of type T in the body, we can only use types
// whose values can be ordered. To enable comparisons, the standard library has
// the std::cmp::PartialOrd trait that you can implement on types. We restrict the
// types valid for T to only those that implements PartialOrd
// both i32 and char implements PartialOrd

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn in_struct() {
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    let both_integer = Point { x: 5, y: 10 };
    println!("{:?}", both_integer);

    #[derive(Debug)]
    struct Point2<T, U> {
        x: T,
        y: U,
    }
    let both_integer = Point2 { x: 5, y: 10 };
    let both_float = Point2 { x: 5.0, y: 10.0 };
    let integer_and_float = Point2 { x: 5, y: 10.0 };
}

fn in_enums() {
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}

fn in_method_definitions() {
    // struct Point<T> {
    //     x: T,
    //     y: T,
    // }

    // // Note that we have to declare T just after impl so we can use T to specify
    // // that we're implementing methods on the type Point<T>. By declaring T as
    // // generic type after impl,
    // impl<T> Point<T> {
    //     fn x(&self) -> &T {
    //         &self.x
    //     }
    // }

    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
}

fn performance_of_code_using_generics() {
    // Using generic types won't make your program run any slower that in would
    // with concrete types.

    // Rust accomplishes this by performing monomorphization fo the code using
    // generics at compile time. Monomorphization is the process of turning
    // generic code into specific code by filling in the concrete types that
    // are used when compiled. In this process, the compiler does the opposite of
    // the steps we use to create the generic function: the compiler looks at all the
    // places where generic code is called and generate code for all the concrete
    // types the generic code is called with.
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = largest(&number_list);
    println!("The largest number is {}", largest);
}
