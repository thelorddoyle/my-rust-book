// standard tasks:
// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

// make the program be able to understand and able to do any of the 3 tasks
// make it so after doing one of the tasks, it returns to the main menu
// rather than just convert celsius to farenheit, make it able to do both based on user requests
// rather than just calc a fibonacci(n), return the vector of all fib numbers AND the fib(n)

fn get_fibonacci_vector(n: u128) -> Vec<u128> {
    let mut sequence = Vec::new();

    let (mut a, mut b) = (0, 1);
    for _ in 0..=n {
        sequence.push(a);
        let temp = a;
        a = b;
        b = temp + b;
    }
    sequence
}

fn get_fibonacci_number(vec: Vec<u128>) -> u128 {
    return vec[vec.len() - 1];
}

fn fibonacci_input() -> u128 {
    println!("Please give a value between 1 and 186 as an integer.");
    let mut string = String::new();
    match std::io::stdin().read_line(&mut string) {
        Ok(_) => match string.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                eprintln!("Error!");
                std::process::exit(0);
            }
        },
        Err(_) => {
            eprintln!("Error!");
            std::process::exit(0);
        }
    }
}

fn number_input_with_prompt(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);

        let mut string = String::new();
        match std::io::stdin().read_line(&mut string) {
            Ok(_) => {
                if string.trim() == "quit" {
                    println!("You have quit the program.");
                    std::process::exit(0);
                }
                match string.trim().parse() {
                    Ok(num) => return num,
                    Err(_) => eprintln!("Error: Input a valid temperature."),
                }
            }
            Err(_) => eprintln!("Error: Failed to read input."),
        }
    }
}

fn convert_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * (5.0 / 9.0)
}

fn convert_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * (9.0 / 5.0)) + 32.0
}

fn converter_tool() {
    println!("Welcome to the temperature converter.");

    println!("Press 1 to convert from Fahrenheit to Celsius.");
    println!("Press 2 to convert from Celsius to Fahrenheit.");

    let choice = number_input_with_prompt("Enter your choice:");

    match choice as i32 {
        1 => {
            let fahrenheit = number_input_with_prompt("Enter the temperature in Fahrenheit:");
            let celsius = convert_to_celsius(fahrenheit);
            println!("The temperature in Celsius is {}.", celsius);
        }
        2 => {
            let celsius = number_input_with_prompt("Enter the temperature in Celsius:");
            let fahrenheit = convert_to_fahrenheit(celsius);
            println!("The temperature in Fahrenheit is {}.", fahrenheit);
        }
        _ => eprintln!("Invalid choice. Enter 1 or 2."),
    }
}

fn get_fibs() {
    let n = fibonacci_input();
    let fib_vec = get_fibonacci_vector(n);
    let fib_nth = get_fibonacci_number(fib_vec.clone());

    println!(
        "Fibonacci vector is: {:?}, Fibonacci number is: {}",
        fib_vec, fib_nth
    );
}

fn print_twelve_days_of_christmas() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Gold Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    for (day_index, day) in days.iter().enumerate() {
        println!("On the {} day of Christmas my true love sent to me:", day);

        for gift_index in (0..=day_index).rev() {
            if day_index > 0 && gift_index == 0 {
                print!("and ");
            }
            println!("{}", gifts[gift_index]);
        }
    }
}

fn print_menu() {
    println!("Choose an option:");
    println!("1 - Temperature converter");
    println!("2 - Fibonacci generator");
    println!("3 - Print the lyrics of 'The Twelve Days of Christmas'");
    println!("4 - Quit");
}

fn main() {
    loop {
        print_menu();

        let choice = number_input_with_prompt("Enter your choice:");

        match choice as i32 {
            1 => converter_tool(),
            2 => get_fibs(),
            3 => print_twelve_days_of_christmas(),
            4 => {
                println!("Goodbye!");
                break;
            }
            _ => eprintln!("Invalid choice. Please enter a number between 1 and 4."),
        }
    }
}
