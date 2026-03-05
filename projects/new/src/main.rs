
fn main() {
    println!("Enter your first name: ");
    let mut first_name = String::new();
    let message = "Try again";
    std::io::stdin()
        .read_line(&mut first_name).expect(&message);
    println!("Enter your last name: ");
    let mut last_name = String::new();
    std::io::stdin()
        .read_line(&mut last_name).expect(&message);
    print!("Hello {} {}",first_name.trim(),last_name.trim());
}
