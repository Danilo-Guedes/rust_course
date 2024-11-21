fn main() {
    println!("\n\n----------------------------\n\n");

    // UNWRAP fn extracts the value from the Option<T> if it is Some<T> and returns the value, is it is None, it panics

    let res = add_strings("Hello", "World").unwrap();

    println!("{res}");

    println!("\n\n----------------------------\n\n");

    // EXPECT fn extracts the value from the Option<T> if it is Some<T> and returns the value, is it is None, it panics with a CUSTOM MESSAGE

    let res2 = add_strings("Hello", "").expect("ERROR: string cannot be empty");

    println!("{res2}");


    println!("\n\n----------------------------\n\n");

    // notice that to confidently use unwrap and expect you need to make SURE your fn or method returns an Option<T> type
    // if not is better to use pattern matching or "?" to make your code more robust and easier to maintain

}

fn add_strings(s1: &str, s2: &str) -> Option<String> {
    if s1.is_empty() || s2.is_empty() {
        return None;
    }

    return Some(format!("{} {}", s1, s2));
}

