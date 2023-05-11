// lets do the traditional and begin with hello world :)
// i am actually instantiating each lesson as a cargo project so this function below came built in to the project

fn main() {
    println!("Hello, world!");
}

// to compile a single rust file use rustc filename
// to run the executable rust file use ./main

// anatomy of a rust program notes

// 1. main function is -always- the first code that is ran in every Rust exe program
// 2. similar to node, params are stored in parentheses () and func body is wrapped in {}
// 3. to format a single rs file can use the rustfmt command
// 4. rust style is to indent with 4 spaces
// 5. println! calls a rust macro as indicated by the !
// 6. similar to nodejs, end lines with semi-colons;
// 7. compile and running are separate steps. compile down to a binary executable then run that exe
// 8. rust is a 'ahead-of-time compiled' language 
// 9. this enables a rust dev to send an exe to someone and they can still run it without having rust installed