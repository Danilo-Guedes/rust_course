fn main() {
    let mut s: String = String::from("Hello");

    let slice: &str = &s; // using &  operator, type is &str

    print_slices_strings(slice);

    let slice2: &str = s.as_str(); // using as_str(), type is &str

    print_slices_strings(slice2);

    let slice3: &str = s.as_mut_str(); // using as_mut_str(), type is &mut str

    print_slices_strings(slice3);

    let s2 = "Hello Danilo";

    let s2_to_string: String = s2.to_string(); // using to_string(), type is String

    print_string_string(s2_to_string);

    let s2_to_string2: String = String::from(s2); // using from(), type is String

    print_string_string(s2_to_string2);
}

fn print_slices_strings(s: &str) {
    println!("{}", s);
}

fn print_string_string(s: String) {
    println!("{}", s);
}
