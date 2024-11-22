use ::std::num::IntErrorKind;
use std::str::FromStr;

fn parse_integer_from_string(input: &str) -> Result<i32, String> {
    match i32::from_str(input) {
        Ok(num) => Ok(num),
        Err(e) => match e.kind() {
            IntErrorKind::Empty => Err("String was empty".to_string()),
            IntErrorKind::InvalidDigit => Err("Invalid digit found".to_string()),
            IntErrorKind::PosOverflow => Err("Positive overflow".to_string()),
            IntErrorKind::NegOverflow => Err("Negative overflow".to_string()),
            IntErrorKind::Zero => Err("Value was zero".to_string()),
            _ => Err("Unknown Error".to_string()),
        },
    }
}

fn main() {
    println!("\n\n-------------\n\n");

    let result = parse_integer_from_string("9");

    match result {
        Ok(num) => println!("Parsed number: {}", num),
        Err(error) => eprintln!("Error parsing number: {}", error),
    }

    println!("\n\n-------------\n\n");

    let result2 = parse_integer_from_string("ABC");

    match result2 {
        Ok(num) => println!("Parsed number: {}", num),
        Err(error) => eprintln!("Error parsing number: {}", error),
    }

    println!("\n\n-------------\n\n");
}
