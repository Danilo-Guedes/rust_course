fn main() {
    println!("{}", convert_kg_to_grams(3.6)); // or you can call convert_kg_to_grams(5 as f64) using casting

    println!("{}", concatenate_strings("Hello", "World"));

    // println!("{}", find_biggest_string("Good", "Morning")); // this will give this error

    //expected named lifetime parameter =
    // help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `s1` or `s2`
    // help: consider introducing a named lifetime parameter
}

fn convert_kg_to_grams(kg: f64) -> f64 {
    return kg * (1000 as f64);
    // can also use return kg * 1000.0
    // or return kg * (1000 as f64)
    // or can ommit the return keyword but in this case must remove the semicolon
}

fn concatenate_strings(s1: &str, s2: &str) -> String {
    s1.to_string() + s2 // implicit return
}

// fn find_biggest_string(s1: &str, s2: &str) -> &str {
//     if s1.len() > s2.len() { s1 } else { s2 }
// }
