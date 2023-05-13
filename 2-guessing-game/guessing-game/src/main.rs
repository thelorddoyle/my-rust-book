// extensions

// the user can set the range DONE
// limit the game to only 3 attempts DONE
// tell the user they can type quit to end the game DONE
// if they write quit as guess, game quits DONE
// include input validation DONE
// ask the user if they want to play again after losing or winning DONE
// handle errors for input DONE
// structure code in to separate functions DONE

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn number_input_with_prompt(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);

        let mut string = String::new();
        match io::stdin().read_line(&mut string) {
            Ok(_) => {
                if string.trim() == "quit" {
                    println!("You have quit the game.");
                    std::process::exit(0);
                }
                match string.trim().parse() {
                    Ok(num) => return num,
                    Err(_) => eprintln!("Error: Input a valid number."),
                }
            }
            Err(_) => eprintln!("Error: Failed to read input."),
        }
    }
}

fn get_range() -> (u32, u32) {
    loop {
        let lower_bound = number_input_with_prompt(
            "In this guessing game, you set the range. What is the lower bound?",
        );
        let upper_bound = number_input_with_prompt("What is the upper bound?");
        // ensure that the lower bound is < upper bound
        if lower_bound < upper_bound {
            return (lower_bound, upper_bound);
        } else {
            eprintln!("Error: Lower bound must be less than the upper bound.");
        }
    }
}

fn guess_number(secret_number: u32) {
    for _attempt in 1..=3 {
        let guess = number_input_with_prompt("Guess the number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
    }

    println!(
        "Sorry, you didn't guess the number in 3 attempts. The number was {}.",
        secret_number
    );
}

fn main() {
    loop {
        // combine both lower and upper bounds in to single return
        let (lower_bound, upper_bound) = get_range();
        let secret_number = rand::thread_rng().gen_range(lower_bound..=upper_bound);

        println!("We have a secret number stored. You have 3 guesses. Type 'quit' to close the game.");
        guess_number(secret_number);

        let play_again = number_input_with_prompt("Do you want to play again? Type 1 for yes, 0 for no.");
        if play_again == 0 {
            eprintln!("Thank you for playing!");
            break;
        }
    }
}