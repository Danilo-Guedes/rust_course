fn main() {
    println!("\n\n--------------------\n\n");

    // PUSH

    let mut vec_of_array_of_numbers = Vec::new();

    vec_of_array_of_numbers.push([1, 2, 3]);
    vec_of_array_of_numbers.push([4, 5, 6]); // from this point we only can push arrays of 3 elements

    println!("vec_of_array_of_numbers: {:?}", vec_of_array_of_numbers);

    println!("\n\n--------------------\n\n");

    // push can move the value into the vector

    let mut names = Vec::new();

    let danilo = String::from("Danilo");

    names.push(danilo);

    // println!("danilo: {}", danilo); // error[E0382]: borrow of moved value: `danilo`

    println!("names: {:?}", names);

    println!("\n\n--------------------\n\n");

    //POP - retrieve last item and remove it from the vector

    let mut vec_of_numbers = vec![1, 2, 3, 4, 5];

    let popped = vec_of_numbers.pop();

    println!("popped: {:?}", popped);

    println!("vec_of_numbers: {:?}", vec_of_numbers);

    println!("\n\n--------------------\n\n");

    //SHINK TO FIT

    let mut vec_with_max_capacity = Vec::with_capacity(8);

    vec_with_max_capacity.push(1);
    vec_with_max_capacity.push(2);
    vec_with_max_capacity.push(3);

    println!(
        "the capacity of vec_with_max_capacity before the srink: {:?}",
        vec_with_max_capacity.capacity()
    );

    vec_with_max_capacity.shrink_to_fit();

    println!(
        "the capacity of vec_with_max_capacity after the srink: {:?}",
        vec_with_max_capacity.capacity()
    );

    println!("\n\n--------------------\n\n");
}
