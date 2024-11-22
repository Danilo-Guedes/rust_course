use std::io::{self, Error, ErrorKind, Write};
use std::str::FromStr;

fn main() {
    println!("\n\n--------------------------\n\n");

    let res = parse_input_to_number();

    match res {
        Ok(num) => println!("the number parsed is: {}", num),
        Err(error) => eprintln!("Error: {:?}", error.kind()),
    }
}

fn parse_input_to_number() -> Result<i32, io::Error> {
    print!("Please enter a number:  ");

    let _ = io::stdout().flush();

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let trimmed_input = input.trim();

    match i32::from_str(&trimmed_input) {
        Ok(num) => Ok(num),
        Err(_err) => {
            Err(Error::from(ErrorKind::InvalidData))
        }
    }
}
