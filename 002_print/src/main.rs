use std::io::Write;

fn main() {
    print!("Hello, "); 
    std::io::stdout().flush().unwrap(); // this line is needed to flush the buffer to preserve the order of the output
    eprintln!("An error occurred: invalid input"); // this line will appear first if the buffer is not flushed
    let name = "Danilo";
    let age = 36;
    let message = format!("My name is {} and I am {} years old", name, age);
    println!("{}", message);
    println!("Hello, World!");
}
