fn main() {
    let mut s1 = String::new();

    s1.push_str("Good");

    let s2 = String::from(" morning");

    // using the add "+" as contactenation operator

    // let s3 = s1 + s2; // Error: cannot add `String` to `String`, String do not implement the `Add` trait

    let s3 = s1 + &s2; // now the add operation works as a concatenation

    println!("{}", s3);

    //println!("{}", s1); // s1 is moved to s3, so it is not available here

    let mut s4 = String::new();

    s4.push_str("Hello, Good ");

    let s5 = "Evening";

    let s6 = format!("{} {}", s4, s5); // format! macro is used to concatenate strings

    println!("{}", s6);

    println!("s4 is not moved, so it is available here : {}", s4); // s4 is not moved, so it is available here

    // we can not add two string literals

    // let s7 = "Hello" + " World"; // Error: cannot add two `&str` strings
}
