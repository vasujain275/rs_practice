use std::io;

fn main() {
    println!("Welcome to VJ Calculator");
    let num1 = read_input("Enter the First Number: ");
    let num2 = read_input("Enter the Second Number: ");
    let op = read_input("Enter the Operator (+,-,*,*)");

    let num1: i32 = match num1.trim().parse() {
        Ok(n) => n,
        Err(err) => {
            println!("Error Parsing the First Number - {err}");
            return;
        }
    };

    let num2: i32 = match num2.trim().parse() {
        Ok(n) => n,
        Err(err) => {
            println!("Error Parsing the Second Number - {err}");
            return;
        }
    };

    let op: char = match op.trim().parse() {
        Ok(n) => n,
        Err(err) => {
            println!("Error Parsing the First Number - {err}");
            return;
        }
    };

    match op {
        '+' => println!("The sum is {}", num1 + num2),
        '-' => println!("The subtracted value is {}", num1 - num2),
        '*' => println!("The product is {}", num1 * num2),
        '/' => println!("The divided result is {}", num1 / num2),
        _ => println!("Please select a vaild operator i.e. +,-,*,/"),
    }
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error taking the input");
    input
}
