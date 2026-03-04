fn main() {
    println!("Hello, world!");
}

// main() is always the first code that runs in every executable Rust program.
// automatic formatter tool - rustfmt
// println! calls a Rust macro by !
// Rust code end with a semicolon

// Compilation and Execution
// After compiling Rust outputs a binary exeutable
// Trade-off in language design:
// Give the executable to someone else, the can run it even without having Rust installed.

// rustc is fine for simple programs, as project grows, you'll want to manage all the options.
// Cargo is thus being used.
