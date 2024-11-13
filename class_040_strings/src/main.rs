fn main() {
    let greeting_as_string_literal = "Hello, world! (string literal)"; // this is a string literal, type: &str, can't be changed

    println!("{greeting_as_string_literal}");

    let greeting_as_string_data_type = String::from("Hello, world! (String data type)");
    // this is a String data type, type: String, can be changed if it's mutable

    println!("{greeting_as_string_data_type}");

    let string_from_method = String::from("Hello, world!  (String::from method)");

    println!("{string_from_method}");

    let string_from_to_string_method = "Hello, world! (to_string method)".to_string();

    println!("{string_from_to_string_method}");

    // every type that implements the Display trait can be converted to a string

    let mut number_to_string = 3.1416.to_string();

    number_to_string.insert_str(0, "Pi: ");

    number_to_string.push_str(" (number to string)");

    println!("{number_to_string}");

    // "shallow copy" or knows as "transfer of ownership" or as "move"

    let string1 = String::from("Hello, world! (shallow copy)");

    let string2 = string1; // this only happens if the data type does not implement the Copy trait

    // println!("{string1}"); // this will cause an error because string1 was moved to string2

    println!("{string2}");

    // deep copy or knows as cloning

    let string3 = String::from("Hello, world! (deep copy)");

    let string4 = string3.clone(); // here we are creating another copy of the data in the heap memory

    println!("{string4}");
}
