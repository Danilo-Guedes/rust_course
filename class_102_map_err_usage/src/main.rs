use std::io::{self, Write};
use std::str::FromStr;

fn main() -> io::Result<()> {
    print!("Please enter a number:  ");

    let _ = io::stdout().flush();

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input)?;

    let res = parse_input_to_number(user_input);

    match res {
        Ok(num) => println!("Parsed Number is: {}", num),
        Err(error) => eprintln!("Error parsing number: {}", error),
    }

    Ok(())
}

fn parse_input_to_number(input: String) -> io::Result<i32> {
    i32::from_str(&input.trim())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{}", e)))
}
