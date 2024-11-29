fn main() {
    println!("\n\n--------------------------\n\n");

    let numbers = vec![1, 2, 3, 4, 5];

    let mut doubled = Vec::new();

    for &number in &numbers {
        doubled.push(number * 2);
    }

    println!("Doubled -> {:?}", doubled);
    println!("Original -> {:?}", numbers);
    println!("\n\n--------------------------\n\n");

    // NOW USING MAP

    let numbers2 = vec![1, 2, 3, 4, 5];

    // the map also return an iterator so we need to collect it

    let doubled2 = numbers.iter().map(|&x| x * 2).collect::<Vec<i32>>();

    println!("Doubled2 -> {:?}", doubled2);
    println!("Original2 -> {:?}", numbers2);
    println!("\n\n--------------------------\n\n");
}
