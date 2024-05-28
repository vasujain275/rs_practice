use std::io;

fn main() {
    println!("Welcome to VJ Calculator");
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut op = String::new();

    println!("Enter the First Number:");
    io::stdin()
        .read_line(&mut num1)
        .expect("Error reading the first number!");

    println!("Enter the second number: ");
    io::stdin()
        .read_line(&mut num2)
        .expect("Error reading the second number!");

    println!("Enter the operator: (+ - * /) ");
    io::stdin()
        .read_line(&mut op)
        .expect("Error reading the operator!");

    let num1: i32 = num1.trim().parse().expect("Error Parsing the number");
    let num2: i32 = num2.trim().parse().expect("Error Parsing the number");
    let op: char = op.trim().parse().expect("Error Parsing the operator");

    match op {
        '+' => println!("The sum is {}", num1 + num2),
        '-' => println!("The subtracted value is {}", num1 - num2),
        '*' => println!("The product is {}", num1 * num2),
        '/' => println!("The divided result is {}", num1 / num2),
        _ => println!("Please select a vaild operator i.e. +,-,*,/"),
    }
}
