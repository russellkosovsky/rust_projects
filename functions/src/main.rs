fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');
}

fn another_function(x: i32) {
    println!("Another function.");
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("Another function.");
    println!("The measurment is: {value}{unit_label}");
}

// statements and expressions

fn main_2() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
// The expression:
// let x = 3;
// x + 1

// Functions with return values

fn five() -> i32 {
    5
}

def main_3() {
    let x = five();
    println!("The value of x is: {x}");
}
