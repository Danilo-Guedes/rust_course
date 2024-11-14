fn main() {
    let words1 = [
        "hello".to_string(),
        "world".to_string(),
        "rust".to_string(),
        "is".to_string(),
        "awesome".to_string(),
    ];

    // iterate by value
    for word in words1 {
        println!("word by value: {}", word);
    }

    //  println!("{:?}", words); //this will ERROR because words has been moved

    println!("\n\n");

    let words2 = [
        "hello".to_string(),
        "world".to_string(),
        "rust".to_string(),
        "is".to_string(),
        "awesome".to_string(),
        "and".to_string(),
        "fun".to_string(),
    ];

    // iterate by reference
    for word in &words2 {
        println!("word by reference: {}", word);
    }

    println!("\n");

    println!("here words2 is still available: {:?}", words2);

    println!("\n\n");

    // iterate by mutable reference
    let mut words3 = [
        "hello".to_string(),
        "world".to_string(),
        "rust".to_string(),
        "is".to_string(),
        "awesome".to_string(),
        "and".to_string(),
        "fun".to_string(),
        "and".to_string(),
        "I".to_string(),
        "will".to_string(),
        "learn".to_string(),
    ];

    //type or word here is &mut String
    for word in &mut words3 {
        word.push_str(" S2");
    }

    println!("words3: {:?}", words3);

    // EXERCISE
    // given a list of numbers, iterate over the list and replace all values less than 10 by the value 0

    let mut numbers = [1, 20, 3, 40, 5, 60, 7, 80, 9, 100];

    for number in &mut numbers {
        if *number <  10 {
            *number = 0;
        }
    }

    println!("numbers after remove less than 10: {:?}", numbers);
}
