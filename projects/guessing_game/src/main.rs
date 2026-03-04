//use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please enter a number");

    let mut guess = String::new();
    

    std::io::stdin()
        .read_line(&mut guess).expect("Failed to read a line");
    
    println!("You guessed {guess}");
    let apples = 3;//immutable, also it's immutable by default
    let mut mut_apples  = 4;

    println!("The immutable apples: {apples}");
    println!("The mutable apples: {mut_apples}");
    //apples = 4; it will cause a problem it must be mut apples since we want to change it later
    mut_apples = 0;
    println!("The immutable apples: {apples}");
    println!("The mutable apples: {mut_apples}");
    //print!();
}
