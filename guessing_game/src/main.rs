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
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100); 
    let mut lives: u32 = 10;
    
    println!("Guessing Game");
    println!("Guess the number, the range is from 1 to 100. You have {lives} lives!");

    loop {
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess_u32: u32 = match guess.trim().parse()  {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse \"{}\" as a number. Please, input a valid number..", guess.trim());
                continue
            }
        };

        println!("You guessed: {guess}");
        match guess_u32.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                lives -= 1;
            }
            Ordering::Greater => { 
                println!("Too big!");
                lives -= 1;
            }
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        }

        println!("You have {lives} lives left!\n");

        if lives == 0 {
            println!("You lost! The secret number is {secret_number}");
            break;
        }
    }

    
}
