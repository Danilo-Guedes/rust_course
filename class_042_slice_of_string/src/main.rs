fn main() {
    let my_string = String::from("hello world");

    let hello = &my_string[0..5]; 

    // hello[0] = 'h'; // error: cannot assign to `hello[..]` because it is borrowed

    // this is not a problem for slices of array for exemple &[i32] if the var is mut you can change the value of the array

    println!("slice: {}", hello);
}
