use std::rc::Rc;

fn main() {
    // All three variables (s1, s2, s3) own the same string data
    // the data won't be dropped until All Rc pointers go out of scope

    let s1 = Rc::new(String::from("Hello, world!"));
    let s2 = Rc::clone(&s1);
    let s3 = Rc::clone(&s1);

    println!("Owner 1: {}", s1);
    println!("Owner 2: {}", s2);
    println!("Owner 3: {}", s3);
}
