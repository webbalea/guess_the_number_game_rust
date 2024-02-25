use std::io;
use std::cmp::Ordering;
use colored::Colorize;
use rand::Rng;


fn main() {
    println!("Guess the number");
    println!("Between 1 and 100");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please enter your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small".red()),
            Ordering::Greater => println!("{}", "Too small".red()),
            Ordering::Equal => {
                println!("{}", "You guessed it right!".green());
                break;
            }
        }
    }
}
