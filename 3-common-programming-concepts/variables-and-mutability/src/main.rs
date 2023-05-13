// const keyword declares a constant 
// and is expected that we use UPPERCASE words separated by _ between words
// I use _ here to indicate we will not use this const value to the compiler
const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let x = 5;

    // here we are 'shadowing' the value of x by declaring the same variable name with let
    // we are destroying the earlier variable in this action
    // it allows us to use the prev value but we are actually creating an entirely new variable
    // this is different from mut, as mut (for example) does not allow any mutation of type whereas shadowing does
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}