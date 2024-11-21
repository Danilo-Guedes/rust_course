fn main() {
    println!("\n\n------------------------\n\n");

    // SPLIT AT

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let (left, right) = numbers.split_at(numbers.len() / 2);

    println!("Left: {:?}", left);
    println!("Right: {:?}", right);

    println!("\n\n------------------------\n\n");

    //SPLIT AT MUT

    let mut numbers_2 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let middle = numbers_2.len() / 2;

    let (left_mut, right_mut) = numbers_2.split_at_mut(middle);

    println!("Left before: {:?}", left_mut);
    println!("Right before: {:?}", right_mut);

    left_mut.iter_mut().for_each(|x| *x *= 10);
    right_mut.iter_mut().for_each(|x| *x *= 10);

    println!("Left after: {:?}", left_mut);
    println!("Right after: {:?}", right_mut);

    println!("\n\n------------------------\n\n");

    // SPLIT

    let numbers_3 = vec![1, 2, 3, 7, 11, 4, 33, 67, 8, 10];


    let subslices: Vec<_> = numbers_3.split(|x| *x % 2 == 0).collect(); // this will split the vector into subslices where the element is even

    for subslice in subslices {
        println!("{:?}", subslice);
    }

    println!("\n\n------------------------\n\n");

    // SPLIT N

    let numbers_4 = vec![1, 2, 3, 7, 11, 4, 33, 67, 8, 10];

    let subslices_2: Vec<_> = numbers_4.splitn(2, |x| *x % 2 == 0).collect(); // this will split the vector into subslices where the element is even, but only 2 times

    for subslice in subslices_2 {
        println!("{:?}", subslice);
    }

    println!("\n\n------------------------\n\n");

    // RSPLIT

    let numbers_5 = vec![1, 2, 3, 7, 11, 4, 33, 67, 8, 10];

    let subslices_3: Vec<_> = numbers_5.rsplit(|x| *x % 2 == 0).collect(); // this will split the vector into subslices where the element is even, but starting from the end

    for subslice in subslices_3 {
        println!("{:?}", subslice);
    }

    println!("\n\n------------------------\n\n");

    //SPLIT OFF

    let mut numbers_6 = vec![1, 2, 3, 7, 11, 4, 33, 67, 8, 10];

    println!("Original vector before: {:?}", numbers_6);

    let split_vec = numbers_6.split_off(5); // this will split the vector at index 5 (similar to pop but from a specific index until the end)

    println!("Original vector after: {:?}", numbers_6);
    println!("Split vector: {:?}", split_vec);


    println!("\n\n------------------------\n\n");

}
