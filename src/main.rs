use std::io;
// Import the `io` module from the standard library for input/output functions

fn main() {
    println!("Guess the number!");

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
