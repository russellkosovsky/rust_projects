
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

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
