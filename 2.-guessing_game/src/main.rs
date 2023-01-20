use std::io;//Import Libraries
use std::cmp::Ordering;
// Some types are imported by the fault in the prelude
// https://doc.rust-lang.org/std/prelude/index.html
use rand::Rng;
// cargo doc --open to see doc files of your dependencies

fn main() {
    println!("Guess the number!"); //Macro(diff from func) to print a line

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");

    loop {
    println!("Please input your guess.");

    // Use let to declare a variable that are INMUTABLE by default
    // If you need a MUTABLE variable you have to specify
    let mut guess = String::new(); // Strings are UTF-8

    io::stdin() // Read from stdin
        .read_line(&mut guess) // Read the user's input in a string. guess is a mutable reference
        .expect("Failed to read line"); //Handles possible error

    // Parse guess from a string to a number 
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num, 
        Err(_) => {
            println!("Plase enter a Positive Number");
            continue;
        }
    };

    println!("You guessed: {guess}"); // Format to print a variable
    // use println!("x = {x} and y + 2 = {}", y + 2); for expression
    //
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => {
                println!("You WIN!!");
                break;
        }
    }
}
}
