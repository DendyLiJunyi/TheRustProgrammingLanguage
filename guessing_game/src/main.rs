/*
prelude is set of items defined in the standard library that it brings into the scope of every program.
*/

// io library from the std library. It provides addtional features.
use std::io;

// Rng refers to the random number generator.
use rand::Rng;

// fn function_name(parameters) {function_body
// }
fn main() {
    // macro prints a string to the screen.
    println!("Guess the number!");

    // thread_rng() is a particular method of Rng
    // gen_range generates a random number in the range.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    // create a mutable variable guess and bound to a new, empty instance of a String.
    let mut guess = String::new();
    // in Rust, variables are immutable by default.
    // add mut will make variable mutable.
    // String::new is a function returns a new instance of a String
    // :: indicates new is an associated function of the String type.
    // associated function is a function implemented on a type.

    io::stdin() // associated function of the io type.
        .read_line(&mut guess)
        // & indicates that this argument is a reference.
        // Can access one piece of data without copy memory.
        // reference is immutable, so one need to write &mut guess.
        // .method_name() syntax.
        .expect("Failed to read line");
    //{} placeholder
    //{} empty placeholder will use a comma-separated list of expressions to print.
    println!("You guessed: {guess}");

    // Semantic Versioning - Really interesting!
    //
}
