use std::io;

fn main() {
    println!("Welcome to the Word Counter!");
    println!("Enter the String - ");
    let mut str = String::new();

    io::stdin()
        .read_line(&mut str)
        .expect("Error reading the string from stdin");

    let arr: Vec<&str> = str.trim().split(" ").collect();
    println!("Word count is {}", arr.len());
}
