fn main() {
    println!("\n\n------------------------\n\n");
    let v_1 = Some(20);
    let _v_2: Option<String> = Some("Danilo".to_string());
    let _v_3: Option<i32> = None; // to use None like this, we need to specify the type
    let _v_4 = Option::<i32>::None; // we also can use the ::None to specify the type

    println!("{:?}", v_1);

    println!("\n\n------------------------\n\n");

    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let found_value = find_value(&array, 8).unwrap_or(-1);

    println!("(Using Unwrap) -> The value 5 is in the index: {}", found_value);

    println!("\n\n------------------------\n\n");

    // we also can use pattern matching to handle the Option type

    let found_value_2 = find_value(&array, 8);

    if let Some(index) = found_value_2 {
        println!("(Using Pattern Matching) -> The value 8 is in the index: {}", index);
    } else {
        println!("(Using Pattern Matching) -> The value 8 is not in the array");
    }

    println!("\n\n------------------------\n\n");
}

fn find_value(array: &[i32], value: i32) -> Option<i32> {
    for (index, item) in array.iter().enumerate() {
        if *item == value {
            return Some(index as i32);
        }
    }
    return None;
}

// FIRST IMPLEMENTATION OF THE FUNCTION NOT USING OPTIONS
// fn find_value(array: &[i32], value: i32) -> i32 {
//     let mut index = 0;
//     for item in array {
//         if *item == value {
//             return index;
//         }
//         index += 1
//     }
//     -1
// }
