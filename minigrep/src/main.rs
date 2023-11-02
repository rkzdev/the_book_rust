use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

// Writing Error Messages to Standard Error Instead of Standard Output

// At the moment, we're writing all of our output to the terminal using the
// println! macro. In most terminals, there are two kinds of output: standard output (stdout)
// for general information and standard error (stderr) for error messages. This
// distinction enables users to choose to direct the successful output of a program
// to a file but still print error messages to the screen.
