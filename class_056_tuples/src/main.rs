fn main() {
    let my_first_typle: (i32, &str, bool) = (42, "Danilo", true);

    println!("{:?}", my_first_typle);

    // values can be acessed with the dot notarion  var.index
    // and the indez cannot be a variable, it has to be a integer

    let number = my_first_typle.0;
    let name = my_first_typle.1;
    let is_valid = my_first_typle.2;

    println!("dot notation => {} - {} - {} ", number, name, is_valid);

    println!("\n\n");

    // also can destructure like other languages

    let (d_number, d_name, d_is_valid) = my_first_typle;

    println!(
        "destructuring => {} - {} - {} ",
        d_number, d_name, d_is_valid
    );

    println!("\n\n");

    // tuples can be returned from a function

    let mut tuples_of_numbers: (i32, i32, i32) = (1, 2, 3);

    println!("before increment {:?}", tuples_of_numbers);

    increment_tuple_values(&mut tuples_of_numbers);

    println!("after increment {:?}", tuples_of_numbers);
}

fn increment_tuple_values(tn: &mut (i32, i32, i32)) {
    tn.0 += 1;
    tn.1 += 1;
    tn.2 += 1;
}
