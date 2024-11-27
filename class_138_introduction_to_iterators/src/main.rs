fn main() {
    println!("\n\n-----------------------\n\n");
    let array_of_numbers = [1, 2, 3, 4, 5];

    for n in array_of_numbers {
        println!("[i32] -> {}", n);
    }

    println!("{:?}", array_of_numbers); // in this case the array is still valid because i32 implements the Copy trait

    println!("\n\n-----------------------\n\n");

    let array_of_strings = [
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
    ];

    for s in array_of_strings {
        println!("[String].into_iter() -> {}", s);
    }

    println!("\n\n-----------------------\n\n");

    // println!("{:?}", array_of_strings);
    // in this case the array is not valid because String does not implement the Copy trait
    // and the ownership of the array is moved to the for loop

    //     |
    // 13  |     let array_of_strings = ["one".to_string(), "two".to_string(), "three".to_string(), "four".to_string(), "five".to_string(),];
    //     |         ---------------- move occurs because `array_of_strings` has type `[String; 5]`, which does not implement the `Copy` trait
    // ...
    // 16  |     for s in array_of_strings {
    //     |              ---------------- `array_of_strings` moved due to this implicit call to `.into_iter()`
    // ...
    // 20  |     println!("{:?}", array_of_strings);
    //     |                      ^^^^^^^^^^^^^^^^ value borrowed here after move

    // BUT WE CAN USER THE .iter() METHOD TO ITERATE OVER THE ARRAY WITHOUT MOVING THE OWNERSHIP how the compiler suggests

    let array_of_strings_2 = [
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
    ];

    for s in array_of_strings_2.iter() {
        println!("[String].inter() -> {}", s);
    }

    println!("{:?}", array_of_strings_2); // in this case the array is still valid b because we used the .iter which does not move the ownership of the array

    println!("\n\n-----------------------\n\n");
}
