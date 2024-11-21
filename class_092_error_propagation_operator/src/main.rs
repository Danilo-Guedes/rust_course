fn main() -> Result<(), AddStringsError> {
    println!("\n\n----------------------------\n\n");

    let res = add_strings("Hello", "World");

    match res {
        Ok(s) => println!("Result: {}", s),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n\n----------------------------\n\n");

    let res2 = add_strings("", "World");

    match res2 {
        Ok(s) => println!("Result: {}", s),
        Err(e) => eprintln!("Error: {}", e),
    }

    println!("\n\n----------------------------\n\n");

    let res3 = add_strings_same_if_size(String::from("Hello"), String::from("World"));

    match res3 {
        Ok(msg) => println!("{msg}"),
        Err(err) => println!("{}", err.description()),
    }

    println!("\n\n----------------------------\n\n");

    let res4 = add_strings_same_if_size(String::from("Danilo"), String::from("Freitas"))?;

    println!("{res4}");

    println!("\n\n----------------------------\n\n");

    Ok(())
}

fn add_strings(s1: &str, s2: &str) -> Result<String, String> {
    if s1.is_empty() {
        return Err("First string is empty".to_string());
    }

    if s2.is_empty() {
        return Err("Second string is empty".to_string());
    }

    Ok(format!("{} {}", s1, s2))
}

fn add_strings_same_if_size(str1: String, str2: String) -> Result<String, AddStringsError> {
    if str1.is_empty() || str2.is_empty() {
        return Err(AddStringsError::EmptyString);
    }
    if str1.len() != str2.len() {
        return Err(AddStringsError::LengthMismatch);
    }

    let result = format!("{} {}", str1, str2);

    Ok(result)
}

#[derive(Debug)]
enum AddStringsError {
    EmptyString,
    LengthMismatch,
}

impl AddStringsError {
    fn description(&self) -> String {
        match self {
            AddStringsError::EmptyString => {
                String::from("Error: found empty string param in add_strings_same_if_size fn")
            }
            AddStringsError::LengthMismatch => String::from(
                "Error: strings params do not have the same length in add_strings_same_if_size fn",
            ),
        }
    }
}
