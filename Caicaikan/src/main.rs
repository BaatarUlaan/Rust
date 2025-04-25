use rand::{Rng, rng};
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    let target = rng().random_range(1..100);
    println!("Random number is {}", target);
    loop {
        println!("Guess a number!");
        let mut guess = String::new();
        
        stdin().read_line(&mut guess).expect("Failed to read line");
        println!("You guessed: {}", guess);
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        match guess.cmp(&target) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
