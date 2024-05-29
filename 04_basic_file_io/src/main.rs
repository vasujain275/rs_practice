use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let in_file = read_in("Enter the input file name (eg- input.txt) - ");
    let out_file = read_in("Enter the output file name (eg- output.txt) - ");
    let contents = fs::read_to_string(in_file)?;
    fs::write(out_file, contents)?;
    Ok(())
}

fn read_in(promt: &str) -> String {
    println!("{}", promt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error taking the user input");
    input.trim().to_string()
}
