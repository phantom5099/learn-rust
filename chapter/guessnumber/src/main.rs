use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("this is a number guessing game!");
    println!("please enter a number:");

    let number = rand::rng().random_range(1..=100);

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        match guess.cmp(&number) {
            Ordering::Less => println!("too low!"),
            Ordering::Greater => println!("too high!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
