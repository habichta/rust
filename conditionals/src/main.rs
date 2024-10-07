use std::{io, process::exit};

fn main() {
    let mut guess = String::new();

    match io::stdin().read_line(&mut guess) {
        Ok(_) => println!("You guessed: {}", guess),
        Err(_) => exit(-1),
    }

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => exit(-1),
    };

    let a = 5;

    if a > guess {
        println!("{a}");
    }
}
