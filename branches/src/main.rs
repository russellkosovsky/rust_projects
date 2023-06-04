fn main() {
    let number = 3;

    if number < 5 {
        println!("Conition was true");
    } else {
        println!("Condition was false");
    }


    println!("---------------------------------------------------------");

    let new_num = 6;

    if new_num % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    

    println!("---------------------------------------------------------");

    let condition = true;
    let num = if condition { 5 } else { 6 };

    println!("The value of num is: {num}");

}
