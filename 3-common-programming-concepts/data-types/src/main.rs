// Rust is a statically typed language
// eg it must know the types of all variables at compile time

// There are four primary scalar types: integers, floating points, booleans and characters

// signed and unsigned integers e.g. i32 or u32
// notes: Each variant can be either signed or unsigned and has an explicit size. Signed and unsigned refer to whether it’s possible for the number to be negative—in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned)
// Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses. So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.

// there are also compound types: two of which are tuples and arrays
// tuples can combine multiple types and once declared cannot grow or shrink in size
// array values are all of the same type but STILL HAVE A FIXED LENGTH
// arrays are useful if you want to store values on the stack rather than in the heap
// a VECTOR is a similar collectiont type that can grow in size
// an example of an array is an array containing strings of all the months in the year

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    // addition
    let sum = 5 + 10;
    println!("{}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("{}", difference);

    // multiplication
    let product = 4 * 30;
    println!("{}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("{}", quotient);
    let truncated = -5 / 3; // Results in -1
    println!("{}", truncated);

    // remainder
    let remainder = 43 % 5;
    println!("{}", remainder);

    // we specify char literals with single quotes, as opposed to string literals, which use double quotes.
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);

    // char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust. Unicode Scalar Values range from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive. However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a char is in Rust

    // tuple example
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // The variable tup binds to the entire tuple because a tuple is considered a single compound element. To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:
    let (x, y, z) = tup;

    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    println!("The value of x is: {x}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("one: {}, five hundred: {}, six point four: {}", one, five_hundred, six_point_four);

    // a tuple () without values is called a UNIT

    // &str is the type, 12 is the number of members in the array
    let _months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:

    let a = [3; 5];
    println!("{:?}", a);

    // final note: if you have an index and the user tries to access the index point that is not possible the program will PANIC at runtime. this is necessary as it avoids the system trying to access a point of invalid memory
}
