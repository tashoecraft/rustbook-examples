// We bring in the rand crate 
extern crate rand;

// Define the libraries we'll be using
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    // In the rand library we can used the methods to generate a random
    // number between 1 - 100
    // (first in incluseive, second is exclusive)
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // infinite loop
    loop {
        println!("Please input your guess.");
        // Define a new, mutable string guess 
        let mut guess = String::new();

        // using stand io, read the input from the cli and assign 
        // it to the mutable variable guess
        // if Result err is thrown, print out the text
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Using a shadow variable, we can modify the guess variable
        // to a trimmed, and converted to 32 bit int. 
        // match will allow us to use Result to catch
        // a Ok condition, to just return num
        // With Err Result to just continue (which will continue the loop)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess: {}", guess);
        // we'll compare the guess to the random number, and
        // use match and Ordering to see which enum value the compare hits and 
        // break if the use succeeds.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Too right!");
                break;
            }

        }   
    }

}
