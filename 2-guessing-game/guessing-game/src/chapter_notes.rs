// guessing game

// import libraries in to scope such as io (the input/output library) using use keyword
// std is the standard library which has sub libraries

// Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude.
// If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement.
// Using the std::io library provides you with a number of useful features, including the ability to accept user input.
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// fn syntax declares a new function, empty parentheses indicate no params
fn main() {
    let x = 5;
    let y = 10;

    // as you can see below, you can use curly brackets as a value placeholder and then after the ',' give the expression to put in place
    println!("x = {x} and y + 2 = {}", y + 2);

    // as discussed in 1.1 println! is a macro
    println!("Guess the number!");

    // start..=end expression for lower and upper bounds
    // running the cargo doc --open command will build documentation provided by all your dependencies locally and open it in your browser
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {}", secret_number);

    loop {
        println!("Please input your guess.");

        // create a variable to store user input
        // let statement creates a variable
        // single = indicates this is a variable and we will bind to what is right of the =
        // on the right of the = is a new string instance
        // String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.
        // The :: syntax in the ::new line indicates that new is an associated function of the String type.
        // An associated function is a function that’s implemented on a type, in this case String. This new function creates a new, empty string.
        let _immutable = String::new(); // immutable, and the _ indicates it is unused on purpose
        let mut guess = String::new(); // mutable due to mut
        io::stdin()
            // this line calls the read_line method on the standard input handle to get input from the user
            // the @mut guess as an argument tells it what string to store the user input in
            // the & operator indicates that this arg is a REFERENCE which gives you a way to let multiple parts of your code access this piece of data without needing to copy in to memory multiple times
            // the return value is a Result<usize, Error>
            // Result is a type that represents either success (Ok) or failure (Err). (from hover)
            // Result is an enum (a type that can be one of multiple possible states)
            // we call each state a VARIANT
            // Result variants purpose is to encode error-handling info
            .read_line(&mut guess)
            // Ok indicates a successful operation
            // Err means op failed and Err contains info about why
            // values of Result type (like values of any type) have methods defined on them
            // an instance of Result has the expect method seen below
            // expect on its own will just crash the program on error but we can further encode error handling typically
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        // up top we bought the io library in to scope, but as it is part of the std lib we could also just use std::io::stdin
        // this is an instance of std::io::Stdin, a type that represents a handle to the standard input for your terminal

        //
        println!("You guessed: {guess}");

        // match is
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

