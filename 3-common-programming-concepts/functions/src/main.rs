// function rules mainly follow the same rules as node, with parameters etc
// similar to TypeScript you need to declare return type

fn main() {
    print_labeled_measurement(5, 'h');

    println!("{}", plus_one(five()).to_string())
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
