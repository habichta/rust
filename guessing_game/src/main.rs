use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    loop {
        println!("Guess the number!");
        println!("Please input your guess.");

        let mut guess: String = String::new();
        let secret_number: u32 = rand::thread_rng().gen_range(0..11);

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        println!("The secret number is: {}", secret_number);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Correct");
                break;
            }
            Ordering::Greater => println!("Too big",),
        }
    }
}
