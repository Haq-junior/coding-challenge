use chrono;
use std::io;

fn main() {
    // TODO: 1. Prompt the user for their name
    println!("What is your name?");

    // TODO: 2. Read the user's input
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim();

    // TODO: 3. Print a personalized greeting
    println!("Hello, {}! Welcome to Rust!", name);

    // BONUS: Print the current date
    let today = chrono::Local::now().date_naive();
    println!("Today's date is: {}", today);
}