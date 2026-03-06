//use std::io;
use std::cmp::Ordering;
//use rand::{Rng};

fn main() {
    /* 
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
    println!("The mutable apples: {mut_apples}");*/

    println!("Guess a number: ");
    let mut _secret_number = rand::random_range(1..=100);
    loop{
        let message = "Failed to read line";
        let mut guess = String::new();

        std::io::stdin()
            .read_line(&mut guess).expect(message);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if guess == _secret_number {
            println!("You won!");
        }else{
            println!("Try later ");
        }
        
        println!("Your guess is {} and secret number is {}"
                    ,guess, _secret_number);

        
            println!("Please input your guess.");
            match guess.cmp(&_secret_number){
                Ordering::Greater => println!("Too big!"),
                Ordering::Less => println!("Too small!"),
                Ordering::Equal =>{ 
                    println!("You won");
                    break;
                }
            }
    }
}
