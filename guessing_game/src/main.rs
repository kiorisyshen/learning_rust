use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let x = 5;
    let y = 10;

    println!("x = {x} and y + 2 = {}", y + 2);

    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_num}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(msg) => {
                println!("{msg}");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
