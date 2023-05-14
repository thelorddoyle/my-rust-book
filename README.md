# My Rust Book

## Description

This is where I am storing my progress through the [Rust Book](https://doc.rust-lang.org/book/).

At each stage I am following along with the code exercises and keeping additional notes, and where appropriate I am extending the exercises in each chapter. In this Readme I will only include the additional steps that I took at each chapter.

_Note: Some chapters are elementary or do not include exercises so are not relevant for additional exercises._

## Additional Exercises

### 2.0 Programming a Guessing Game

**Standard Task**

1. Create the requested guessing game in the code-along

**Additional Self-Set Tasks**

1. The user can set the range
2. Limit the game to only 3 attempts
3. Update the User Experience, so the user understands the rules
4. If they write 'quit' as guess, game quits
5. Include input validation to ensure lower bound is lower than the upper bound
6. Ask the user if they want to play again after losing or winning
7. Correctly handle errors for input
8. Structure code in to separate functions

Completion of standard tasks and additional tasks for Chapter 2 can be found [here](https://github.com/thelorddoyle/my-rust-book/blob/main/2-guessing-game/guessing-game/src/main.rs)

### 3.0 Common Programming Concepts

**Standard Tasks**

1. Convert temperatures between Fahrenheit and Celsius.
2. Generate the nth Fibonacci number.
3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

**Additional Self-Set Tasks**

1. Produce a main menu and ask the user which of the 3 tasks they would like the program to conduct
2. After doing one of the tasks, it returns to the main menu
3. Rather than just convert celsius to farenheit, make it able to do both based on user request
4. Rather than just calc a fibonacci(n), return the vector of all fib numbers AND the fib(n) up to the 186th Fibonacci number (u128)

Completion of standard tasks and additional tasks for Chapter 3 can be found [here](https://github.com/thelorddoyle/my-rust-book/blob/main/3-common-programming-concepts/control-flow/src/main.rs)
