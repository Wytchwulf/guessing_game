use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // Generate Random number between 1 & 100

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        // Create a new, empty String and assign it to a mutable variable `guess`

        io::stdin()
            .read_line(&mut guess)
            // Reads a line of input from standard input and appends it to `guess`
            // `&mut guess` passes a mutable reference so the function can modify it
            .expect("Failed to read line");
        // If reading fails, print this error and crash

        let guess: u32 = guess.trim().parse().expect("Please type a number");
        // Shadow guess variable.
        // Remove whitespace with trim.
        // Parse to convert string to type u32

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win!!");
                break;
            }
        }
    }
}
