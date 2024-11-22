fn main() {
    println!("\n\n----------------------------\n\n");

    // UNWRAP_ERR is the counterpart of unwrap, it panicts if the result is a OK variant and returns it

    let res = add_strings("Hello", "").unwrap_err();

    println!("{res}");

    println!("\n\n----------------------------\n\n");

    // UNWRAP_OR  it returns the value if the result is a OK variant, otherwise it returns the value passed as argument

    let res2 = add_strings("Hello", "").unwrap_or("ERROR: string cannot be empty".to_string()); 

    println!("{res2}");

    println!("\n\n----------------------------\n\n");
}

fn add_strings(s1: &str, s2: &str) -> Result<String, String> {
    if s1.is_empty() || s2.is_empty() {
        return Err("ERROR: string cannot be empty".to_string());
    }

    return Ok(format!("{} {}", s1, s2));
}
