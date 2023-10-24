fn main() {
    defining_and_instantiating();

    tuple_structs();

    unit_like_structs();

    ownership_of_struct_data();

    program_using_struct();

    method_syntax();

    method_with_more_parameters();

    associated_function();
}

fn associated_function() {
    // All functions defined an impl block are called associated function because they're
    // associated with the type named after the impl. We can define associated functions that don't
    // have self as their first parameter (and thus are not methods) because they don't need an
    // instance of the type to work with. We've already used one function like this: the
    // String::from function that's defined on the String type.

    // Associated functions that aren't methods are often used for constructor that will return a
    // new instance of the struct. These are often called new, but new isn't a special name and
    // isn't built into the language. For example, we could choose to provide an associated
    // function named square that would have one dimension parameter and use that as both width and
    // height, thus making it easier to create a square Rectangle rather than having to specify the
    // same value twice:

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    // The Self keyword in the return type and in the body of the function are aliases for the type
    // that appears after the impl keyword, which is this case is Rectangle.

    // To call this associated function, we use the :: syntax with the struct name; let sq =
    // Rectangle::square(3); is an example. This function is namespaced by the struct: the ::
    // syntax is used for both associated functions and namespaces created by module.

    let sq = Rectangle::square(30);
    println!("{:#?}", sq);
}

fn method_with_more_parameters() {
    struct Rectangle {
        width: u32,
        height: u32,
    }

    #[allow(dead_code)]
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, o: &Rectangle) -> bool {
            self.width > o.width && self.height > o.height
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rectt2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rectt3? {}", rect1.can_hold(&rect3));
}

fn method_syntax() {
    // Methods are similar to functions; we declare them with the fn keyword and a name, they can
    // have parameters and return value, and they contain some code that's run when the method is
    // called from somewhere else. Unlike functions, methods are defined within the context of a
    // structt (or an enum or a trait object), and their first parameter is always self, which
    // represents the instance of the struct the method is being called on.

    // Defining Methods
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    // To define the function within the conext of Rectange, we start an impl (implementation)
    // block for Rectangle. Everything within this impl block will be associated with the Rectangle
    // type. Then we move the area function within the impl curly brackets and change the first
    // parameter to be self in the signature and everywhere within the body.

    // In the signature for area, we use &self instead of rectangle: &Rectangle. The &self is
    // actually short for self: &Self. Within an impl block, the type Self is an alias for the type
    // that the impl block is for. Methods must have a parameter named self of type Self for their
    // first parameter, so Rust lets you abbreviate this with only the same self in the first
    // parameter spot. Note that we still need to use the & in front of the self shorthand to
    // indicate tthat this method borrows the Self instance, just as we did in rectangle:
    // &Rectangle. Methods can take ownership of self, borrow self immutably, as we've done here,
    // or borrow self mutably, just as they can any other parametter.

    // Here, we're choosing to make the width method return ture if the value in the instance's
    // width field is greater than 0 and false if the value is 0: we can use a field within a
    // method of the same name for any purpose.

    // Often, but not always, when we give a method the same name as field we want it to only
    // return the value in the field and do nothing else. Methods like this are called getters, and
    // Rust does not implement them automatically for struct fields as some other languages do.
    // Getters are useful because you can make the field private but the method public, and thus
    // enable read-only access to that field as part of the type's public API.

    // Where's the -> Operator?
    // In C and C++, two different operators are used for calling methods: you use . if you're
    // calling a method on the object directly and -> if you're calling the method on a pointer to
    // the object and need to dereference the pointer first. In other words, if object is a pointer
    // object->something() is similar to (*object).something().

    // Rust doesn't have an equivalent to the -> operator; instead, Rust has a feature called
    // automatic referencing and dereferencing. Calling methods is one of the few places in Rust
    // that has this behavior.

    // Here's how it works: when you call a method with object.something(), Rust automatically adds
    // in &, &mut, or * so object matches the signature of the method. In other words, the
    // following are the same:
    // p1.distance(&p2);
    // (&p1).distance(&p2);

    // The first one looks much cleaner. This automatic referencig behavior works because methods
    // have a clear receiver - the type of self. Given the receiver and name of a method, Rust can
    // figure out definitively whether the method is reading (&self), mutating (&mut self), or
    // consuming (self). The fact that Rust makes borrowing implicit for method receivers is a big
    // part of making ownership ergonomic in practice.

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn width(&self) -> bool {
            self.width > 0
        }
    }

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The are of the rectangle is {} square pixels.",
        rect1.area()
    );

    // When we follow rect1.width with parentheses, Rust knows we mean the method width.
    // When we don't use parentheses, Rust knows we mean the field width.
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}

#[allow(dead_code)]
fn program_using_struct() {
    // let width = 30;
    // let height = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width, height)
    // );

    // fn area(width: u32, height: u32) -> u32 {
    //     width * height
    // }

    // let rect1 = (30, 50);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // );

    // fn area(dimensions: (u32, u32)) -> u32 {
    //     dimensions.0 * dimensions.1
    // }

    // struct Rectangle(u32, u32);

    // let rectangle = Rectangle(30, 50);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(&rectangle)
    // );

    // fn area(shape: &Rectangle) -> u32 {
    //     shape.0 * shape.1
    // }

    #[derive(Debug)]
    struct Rectangle {
        height: u32,
        width: u32,
    }

    let rect1 = Rectangle {
        height: 30,
        width: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    // Note: Calling db! macro prints to the standard error console stream (sderr), as opposed to
    // println!, which prints to the standard output console stream (stdout)
    // We can put dbg! around the expression 30 * scale and, because dbg! returns ownership of the
    // expression's value, the width field will get the same value as if we didn't have the dbg!
    // call there. We don't want dbg! to take ownership of rect1, so we use a reference to rect1 in
    // the next call. Here's what the output of this example looks like:

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}

#[allow(dead_code)]
fn ownership_of_struct_data() {
    // Ownership of Struct Data

    // In the User struct definition, we used the owned String type rather than the &str string slice
    // type. Tthis is a deliberate choice because we want each instance of thtis struct to own all of
    // its data and for that data to be valid for as long as the entire struct is valid.

    // It's also possible for struct to store references to data owned by something else, but to do so
    // requireds the use of lifetimes, a Rust feature. Lifetimes ensure that the data referenced by a
    // struct is valid for as long as the struct is.

    // The compiler will complain that it needs lifetime specifiers:
    // struct User {
    //     active: bool,
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    // }
}

#[allow(dead_code, unused_variables)]
fn unit_like_structs() {
    // You can also define structs that don'tt have any fields! These are called unit-like structs
    // because they behave similarly to (). Unit-like structs can be useful when you need to
    // implement a trait on some type but don't have any data that you want to store in the type
    // itself.

    struct AlwaysEqual;

    let subject = AlwaysEqual;

    // To define AlwaysEqual, we use the struct keyword, the name we want, and then a semicolon. No
    // need for curly brackets or parenthesis! Tthen we can get an instance of AlwaysEqual in the
    // subject variable in a similar way: using the name defined, without any curly brackets or
    // parentheses. Imagine that later we'll implement behavior for this type such that every
    // instance of AlwaysEqual is always equal to every instance of any other type, perhaps to have
    // a known result for testing purposes.
}

#[allow(dead_code)]
fn tuple_structs() {
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{:?}", black);
    println!("r: {}, g: {}, b: {}", black.0, black.1, black.2);
    println!("{:?}", origin);
    println!("x: {}, y: {}, z: {}", origin.0, origin.1, origin.2);
}

#[allow(dead_code)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[allow(dead_code)]
fn defining_and_instantiating() {
    let mut user1 = User {
        active: true,
        username: String::from("rkzphdev"),
        email: String::from("rkz.phdev@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("username");

    println!("{}", user1.email);

    let user2 = build_user(
        String::from("ravenp.dev@gmail.com"),
        String::from("ravenpdev"),
    );
    println!("User2: {:?}", user2);

    // Struct update syntax
    let user3 = User {
        email: String::from("exampl@gmail.com"),
        ..user2
    };
    println!("Using pretty print: {:#?}", user3);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

#[allow(dead_code)]
fn intro() {
    // Using Structs to Structure Related Data

    // A struct, or structure, is a custom data type that lets you package together and name
    // multiple related values that make up a meaningful group. If you're familiar with an
    // object-oriented language, a struct is like an object's data attributes. We will compare and
    // contrast tuples with structs to build on what you already know and demonstrate when structs
    // are a better way to group data.

    // We'll demonstrate how to define and instantiate structs. We'll discuss how to define
    // assoicated functions, especially the kind of associated functions called methods, to specify
    // behavior associated with a struct type. Structs and enums are the building blocks for
    // creating new types in your program's domain to take full advantage of Rust's compile-time
    // type checking.
}
