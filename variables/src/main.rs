
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// VARIABLES
fn main() {
    println!("there are {THREE_HOURS_IN_SECONDS} seconds in three hours.");
    
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    let spaces = "  ";
    {
        let spaces = spaces.len();
        println!("there are {spaces} spaces.");
    }
    println!("spaces: {spaces}!")
} 

fn floating_point_types() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}

fn boolean_types() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}

fn character_type() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}

fn tuple_type() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
}

fn array_type() {
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // with explicit type annotation
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    let first = a[0];
    let second = a[1];
}

use std::io;

fn invalid_array_access() {
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

