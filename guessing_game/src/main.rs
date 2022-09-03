use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main() {
    println!("Guess the number!");
    let secrete_number = rand::thread_rng().gen_range(0..101);
    println!("The secrete number is: {}", secrete_number);

    loop {
        println!("Input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {}", guess);

        match guess.cmp(&secrete_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too high!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            },
        };
    }
}

