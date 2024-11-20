fn main() {
    println!("\n\n----------------- -\n\n");

    let v = vec![1, 2, 3, 4, 5];

    println!("The third element of v is {}", v[2]); // in this case i32 implements Copy trait then the value is copied

    // println!("{},", v[15]) // panic! because the index is out of bounds

    println!("\n\n----------------- -\n\n");

    let v_strings = vec!["Sun".to_string(), "Mon".to_string(), "Tue".to_string()];

    // let first_day = v_strings[0];// this will error: cannot move out of index of `Vec<String>` because String does not implement Copy trait

    let first_day = &v_strings[0]; // this will work because we are borrowing the value

    println!("The first day of the week is {}", first_day);

    println!("\n\n----------------- -\n\n");

    // the safest way to access an element of a vector is using the get and get_mut methods

    let mut v3 = vec![10, 20, 30, 44, 50];

    match v3.get(2) {
        Some(third) => println!("The third element of v3 is {}", third),
        None => println!("There is no third element."),
    }

    println!("\n\n----------------- -\n\n");

    let last_v3_index = v3.len() - 1;

    let last_item_to_be_modified = v3.get_mut(last_v3_index);

    if let Some(last) = last_item_to_be_modified {
        *last *= 10;
    } else {
        println!("There is no last element.");
    }

    println!("the v3 after modification is {:?}", v3);

    println!("\n\n----------------- -\n\n");
}
