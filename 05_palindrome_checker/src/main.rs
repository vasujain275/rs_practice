use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter the word you want to check - ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading the user input");
    let rev_input: Vec<char> = input.trim().chars().rev().collect();
    let input: Vec<char> = input.trim().chars().collect();
    let result: bool = input.eq(&rev_input);

    match result {
        false => {
            println!("The word is not a palindrom");
        }
        true => {
            println!("The word is a palindrom");
        }
    }
}
