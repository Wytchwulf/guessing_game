// Import external crates and modules
use rand::Rng; // For generating random numbers
use std::cmp::Ordering; // For comparing values (Less, Greater, Equal)
use std::io; // For reading user input

fn main() {
    println!("Guess the number!");
    // Print a welcome message when the program starts.

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // Create a random number between 1 and 100 (inclusive).
    // `thread_rng()` gives a random number generator specific to this thread.
    // `gen_range(1..=100)` means random number from 1 up to and including 100.

    let mut attempts = 0;
    // Initialize variable to log the number of attmpts.

    loop {
        // Start an infinite loop.
        // The game will keep asking until the player guesses correctly.

        println!("Please input your guess");
        // Prompt the user to enter a guess.

        let mut guess = String::new();
        // Create a new, empty String to store the user's guess.
        // `mut` makes it mutable so we can change it.

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Read a line of input from the terminal and put it into `guess`.
        // If it fails, crash the program with an error message.

        attempts += 1;
        // Increase the guess attempt counter

        if guess.trim().eq_ignore_ascii_case("quit") {
            println!("Goodbye");
            break;
        }
        // If user types quit, exit the game.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // Shadow the old `guess` by creating a new `guess` variable.
        // - `trim()` removes whitespace and newline characters.
        // - `parse()` tries to convert the trimmed string to a u32 number.
        // - `match` handles success or failure:
        //     - If successful (`Ok(num)`), use the number.
        //     - If failed (`Err(_)`), skip this loop and ask again.

        println!("You guessed {}", guess);
        // Show what number the user guessed.

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!! It took you {} attempts!", attempts);
                break;
            }
        }
        // Compare the guess to the secret number:
        // - If guess is smaller, print "Too Small!"
        // - If guess is bigger, print "Too Big!"
        // - If guess is correct, print "You Win!!" and exit the loop.
    }
}
