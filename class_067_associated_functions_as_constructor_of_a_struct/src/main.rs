fn main() {
    println!("\n--------------------\n");
    let rect = Rectangle::new(30, 50);
    println!("rect is {:?}", rect);
    println!("\n--------------------\n");

    let empty = Rectangle::new_empty();
    println!("empty is {:?}", empty);
}

#[derive(Debug, Default)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle { // could replace this return type with "Self" which is the type of the struct 
    // also notice that there is no self parameter here, this is an associated function, not a method
        Rectangle { width, height }
    }

    fn new_empty() -> Rectangle {
        Rectangle::default()
    }
}
