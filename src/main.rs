use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {}", secret_number);

    println!("Please input your guess");

    let mut guess = String::new();
    // Create a new, empty String and assign it to a mutable variable `guess`

    io::stdin()
        .read_line(&mut guess)
        // Reads a line of input from standard input and appends it to `guess`
        // `&mut guess` passes a mutable reference so the function can modify it
        .expect("Failed to read line");
    // If reading fails, print this error and crash

    println!("You guessed {}", guess);
}
