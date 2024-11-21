fn main() {
    println!("\n\n--------------------\n\n");

    //RETAIN

    let mut numbers = vec![-1, 2, -3, 4, -5, 6, -7, 8, -9, 10];

    println!("Original numbers: {:?}", numbers);

    numbers.retain(|x| *x > 0);

    println!("Positive numbers: {:?}", numbers);

    println!("\n\n--------------------\n\n");

    // RETAIN MUT

    let mut numbers_2 = vec![-1, 2, -3, 4, -5, 6, -7, 8, -9, 10];

    println!("Original numbers: {:?}", numbers_2);

    numbers_2.retain_mut(retain_and_modify_positives);

    println!("Positive and changed numbers : {:?}", numbers_2);

    println!("\n\n--------------------\n\n");
}

fn retain_and_modify_positives(x: &mut i32) -> bool {
    if *x > 0 {
        *x *= 10;
        true
    } else {
        false
    }
}
