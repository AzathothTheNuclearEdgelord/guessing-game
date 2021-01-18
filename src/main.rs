use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess a number somewhere between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{
        println!("Toss your guess in here.");

        // Define new varaible called "guess"
        let mut guess = String::new();

        // Get input from user
        io::stdin()
          .read_line(&mut guess)
          .expect("Failed to read line");

        // Ensure the input is a number and convert it to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("\nYou guessed: {}", guess);

        // Match the "guess" number with secret_number and print a suitable message based on that
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
