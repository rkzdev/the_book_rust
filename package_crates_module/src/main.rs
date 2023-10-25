use crate::garden::vegetables::Asparagus;

// the pub mod garden; line tells the compiler to include the code it finds in src/garden.rs
pub mod garden;

fn main() {
    // As a project grows you should orgnaize code by splitting it into multitple modules and
    // mutilple files. A package can contain multitple binary crates and optitonally one library
    // create. As a package grows, you can extract parts into separate crates that become external
    // dependencies.

    // Rust has a number of features that allow you to manage your code's organization, including
    // which details are exposed, which details are private, and what names are in each scope in
    // your program. These features, sometimes collectively referred to as the module system,
    // include:

    // Package - A Cargo feature that lets you build, test, and share crates
    // Crates - A tree modules that produces a library or executable
    // Modules and use - Let you control the organization, scope, and privacy paths
    // Paths - A way of naming item, such as struct, functions, or module

    packages_and_crates();

    modules_to_control_scope_and_privacy();

    grouping_related_code_in_modules();
}

fn paths_for_referring_an_item_in_the_module_tree() {
    // To show Rust where to find an item in a module tree, we use a path in the same way
    // we use a path when navigating a filesystem. To call a function, we need to know its
    // path.

    // A path can take two forms:
    // 1. An absolute path is the full path starting from a crate root; for code from an
    // external crate, the absolute path begins with the crate name, and for code from the
    // crate, it starts with the literal crate.
    // 2. A relative path starts from the current module and uses self, super, or an identifier
    // in the current module.

    // Both absolute and relative paths are follwed by one or more indentifiers separated
    // by double colons (::)
}

fn grouping_related_code_in_modules() {
    // Modules let us orgnaize code within a crate for readability and easy resuse. Modules
    // also allow us to control the privacy of items, because code within a module is a private
    // by default. Private items are internal implmentationdetails not available for outside use.
    // We can choose to make modules and the items within them public, which exposes them
    // to allow external code to use and depend of them.
}

fn modules_to_control_scope_and_privacy() {
    // Paths allow you ti name items; the use keyword that brings a path into scope;
    // and the pub keyword to make items public.

    // Here we provide a quick reference on how modules, path, the use keyword
    // and the pub keyword work in the compiler, and how most developers organized
    // their code.

    // 1. Start from the crate root: When compiling a crate, the compiler first look in
    // the crate root file for code to compile.

    // 2. Declaring modules: In the crate root, you can declare a new modules; say
    // you declare a "garden" module with mod garden; the compiler will look for the
    // module's code in these places:
    // - Inline, within curly brackets that replace the semicolon mod garden
    // - In the file src/garden.rs
    // - In the file src/garden/mod.rs

    // 3. Declaring submodules: In any file other than the crate root, you can declare
    // submodules. For example, you might declare mod vegetables; in src/garden.rs. The
    // compiler will look for the submodule's code within the directory named for the parent
    // module in these places:
    // - Inline, directory following mod vegetables, within curly brackets instead of the
    // semicolon
    // - In the file src/garden/vegetables.rs
    // - In the file src/garden/vegetables/mod.rs

    // 4. Paths to code in modules: Once a module is part of your crate, you can refer to
    // code in that module from anywhere else in that same create, as long as the privacy
    // rules allows, using the path to the code. For example, an Asparagus typein the garden
    // vegetables module would be found at crate::garden::vegtables::Asparggus.

    // 5. Private vs public: Code within a module is private from its parent modules by default.
    // To make a module public, declare it with pub mod instead of mod. To make items within
    // a public module public as well, use pub before their declarations.

    // 6. The use keyword: Within a scope, the use keyword creates shortcuts to items to reduce
    // repetition of long paths. In any scope that can refer to
    // crate::garden::vegetables::Asparagus, you can create a shortcut with use
    // crate::garden::vegetables::Asparagus; and from then on you only need to write Asparagus
    // to make use of thath type in the scope.

    let plant = Asparagus {};
    println!("I'm growing {:#?}", plant);
}

fn packages_and_crates() {
    // A crate is the smallest amount of code that the rust compiler considers at a time.
    // Even if you run rustc rather than cargo and pass a single source code file
    // the compiler considers that file to be a crate.

    // A crate can come in one of two forms: a binary crate or a library crate.
    // Binary crates are programs you can compile to an executable that you can
    // run, such as command-line program or a server. Each must have a function
    // called main that defines what happens when the executable runs.

    // Library crates don't have a main function, and they don't compile to an
    // executable. Instead, they define functionality intended to be shared with
    // multiple projects. For example, the ran create provides functionality that
    // generate random nubers. Most of the time when rustaceans say "crate", they
    // mean library create, and they use "crate" interchangeably with the general
    // programming concept "library".

    // The crate root is a source file that the rust compiler starts form and makes
    // up the root module of your crate.

    // A package is a bundle of one or more crates that provides a set of functionality.
    // A package contains a Cargom.toml file that describes how to build those crates.
    // Cargo is actually a package that contains the binary crate for the command-line
    // tool you've been using to build your code. The Cargo package also contains a
    // library crate that the binary crate depends on.

    // A package can contain as many binary crates as you like, but at most only one
    // library crate.

    // Cargo follows a convention that src/main.rs is the crate root of binary crate
    // with the same name as the package. Likewise, Cargo know tthat if the package
    // directory contains src/libs.rs, the package contains a library crate with the
    // same name as the package, and src/lib.rs is its crate root. Cargo passes the
    // crate root files to tustc to build the library or binary.
}
