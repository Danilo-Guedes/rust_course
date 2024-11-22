fn parse_integer_from_string(input: &str) -> Result<i32, String> {
    let res = input.parse::<i32>();

    if res.is_err() {
        let error = res.unwrap_err();

        return Err(format!("{:?}", error.kind()));
    } else {
        Ok(res.unwrap())
    }
}

fn main() {
    println!("\n\n-------------\n\n");

    let result = parse_integer_from_string("9");

    match result {
        Ok(num) => println!("Parsed number: {}", num),
        Err(error) => eprintln!("Error: {}", error),
    }

    println!("\n\n-------------\n\n");

    let result2 = parse_integer_from_string("ABC");

    match result2 {
        Ok(num) => println!("Parsed number: {}", num),
        Err(error) => eprintln!("Error : {}", error),
    }
}
