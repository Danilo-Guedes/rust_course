fn main() {
    println!("\n\n-------------------------\n\n");

    let mut numbers = [1, 2, 3, 4, 5];

    for i in 0..numbers.len() {
        numbers[i] *= numbers[i];
    }

    println!("using for loop {:?}", numbers);

    println!("\n\n-------------------------\n\n");

    // FOR EACH

    let mut numbers2 = [1, 2, 3, 4, 5];

    numbers2.iter_mut().for_each(|n|  *n *= *n );

    println!("using for_each {:?}", numbers2);

    println!("\n\n-------------------------\n\n");
}
