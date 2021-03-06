extern crate rand;

use std::io;
use std::cmp::Ordering; // Ordering enum
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("The secret number is: {}", secret_number);
    let mut n: u32 = 0;
    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // To Input
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line"); // Error message
        println!("You Guessed: {}", guess);

        // Use option
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win for {} times", n);
                break;
            }
        }
        n += 1;
    }
}