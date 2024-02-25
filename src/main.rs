use std::io;
use std::cmp::Ordering;
use colored::Colorize;
use rand::Rng;

fn main() {
    println!("Guess the number!\nBetween 1 and 100.");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        // Better user prompt.
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Invalid number! Please enter a number.".yellow());
                continue
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".bright_red()),  //Improve the visibility of text
            Ordering::Greater => println!("{}", "Too big!".bright_red()), //Correct text to "Too big!"
            Ordering::Equal => {
                println!("{}", "You guessed it right!".bright_green());
                break;
            }
        }
    }
}