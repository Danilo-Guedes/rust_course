fn extend_string<'a>(original: &'a mut String, data: &'a str) -> &'a str {
    original.push_str(" ");
    original.push_str(data);
    original
}

fn main() {
    println!("\n\n-------------------------\n\n");
    let mut original = String::from("hello");
    let result = extend_string(&mut original, "world");
    println!("the result {}", result);
    println!("\n\n-------------------------\n\n");

}

