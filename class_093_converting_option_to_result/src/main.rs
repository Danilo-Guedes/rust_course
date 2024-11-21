fn main() -> Result<(), String> {
    println!("\n\n----------------------------\n\n");

    // OR_OR transform a Option<T> into a Result<T, Err>

    let res = add_strings("Hello", "World").ok_or("ERROR: strings cannot be empty!")?;

    println!("{res}");

    println!("\n\n----------------------------\n\n");

    let res2 = add_strings("Hello", "").ok_or("ERROR: strings cannot be empty!")?;

    println!("{res2}");


    println!("\n\n----------------------------\n\n");

    Ok(())

}

fn add_strings(s1: &str, s2: &str) -> Option<String> {
    if s1.is_empty() || s2.is_empty() {
        return None;
    }

    return Some(format!("{} {}", s1, s2));
}

