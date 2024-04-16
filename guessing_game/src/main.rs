use std::io;
use std::cmp::Ordering;
use rand::Rng;

/*
    Guessing Game
    The program will generate a random integer between 1 and 100. 
    It will then prompt the player to enter a guess. 
    After a guess is entered, the program will indicate whether the guess is 
    too low or too high. If the guess is correct, 
    the game will print a congratulatory message and exit.
*/

// To Do - add number or retries
// To Do - do not reduce the number of retries when out of range

fn main() {
    // start..=end - inclusive on the lower and upper bounds
    let secret_number = rand::thread_rng().gen_range(1..=100); 

    println!("Guessing Game");
    println!("Guess the number, range 1 to 100!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess_u32: u32 = guess.trim().parse()
        .expect(&format!("Failed to parse \"{}\" as a number", guess.trim()));

    println!("You guessed: {guess}");
    match guess_u32.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("The secret number is {secret_number}");
}
