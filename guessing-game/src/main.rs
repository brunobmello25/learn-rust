use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("--- Guess the number! ---\n");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let guess = match get_guess() {
            Ok(guess) => guess,
            Err(_) => {
                println!("Please input a number!\n");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!\n");
                break;
            }
        }
    }
}

fn get_guess() -> Result<u32, std::num::ParseIntError> {
    println!("please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    return guess.trim().parse();
}
