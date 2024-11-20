#![feature(extract_if)] // to this use the "cargo +nightly run" command

fn main() {
    println!("\n\n---------------------\n\n");

    // DRAIN

    let mut v = vec![1, 2, 3, 4, 5];

    v.drain(1..3); // this will remove the elements at index 1 and 2 (it's like a pop but for multiple elements)

    println!("{:?}", v);

    println!("\n\n---------------------\n\n");

    let mut v2 = vec![1, 2, 3, 4, 5];

    println!("v2 before drain {:?}", v2);

    let mut v_extracted = vec![];

    v_extracted.extend(v2.drain(1..3)); // this will remove the elements at index 1 and 2 and return them as a new vector

    println!("v2 after drain {:?}", v2);

    println!("v_extracted {:?}", v_extracted);

    println!("\n\n---------------------\n\n");

    // EXTRACT IF

    let mut numbers = vec![1, 2, 3, 4, 5, 6, 8, 9, 11, 13, 14, 15];

    let evens = numbers.extract_if(|x| *x % 2 == 0).collect::<Vec<_>>();
    let odds = numbers;

    println!("evens {:?}", evens);
    println!("odds {:?}", odds);

    println!("\n\n---------------------\n\n");
}
