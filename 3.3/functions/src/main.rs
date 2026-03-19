fn main() {
    print_labeled_measurement(5, 'h');
    println!("The value is: {}", five());
    println!("The value 5+1 is: {}", plus_one(5));
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
