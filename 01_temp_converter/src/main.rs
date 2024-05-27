use std::io;

fn main() {
    println!("Welcome to Blazingly fast Calculator!");
    println!("Enter the Unit of Temperature (C/F): ");
    let mut unit = String::new();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut unit)
        .expect("Error reading the unit");

    println!("Enter the value: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading the value");

    let input: f32 = input
        .trim()
        .parse()
        .expect("Error Parsing the input to integer");

    if unit.trim().to_uppercase().eq("C") {
        println!("Converted is - {}F", ((input * 1.8) + 32.0));
    }

    if unit.trim().to_uppercase().eq("F") {
        println!("Converted is - {} C", ((input - 32.0) * 0.56));
    }
}
