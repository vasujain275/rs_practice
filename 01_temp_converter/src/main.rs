use std::io;

enum Format {
    C(i32),
    F(i32),
}

fn main() {
    println!("Welcome to Blazingly fast Calculator!");
    println!("Enter the Unit of Temperature (C/F): ");
    let mut opt = String::new(); // Defaults to Celcius
    match io::stdin().read_line(&mut opt) {
        Ok(..) => {
            if opt.to_uppercase().eq("C") {
                println!("YES")
            }
        }
        Err(error) => {
            println!("error : {error}")
        }
    }
}
