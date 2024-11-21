use std::collections::HashMap;
fn main() {
    println!("\n\n-----------------------\n\n");

    // AND_MODIFY

    let mut fruits = HashMap::new();

    fruits
        .entry("apple")
        .and_modify(|value| *value += 1)
        .or_insert(2);

    println!("{:?}", fruits);

    fruits
        .entry("apple")
        .and_modify(|value| *value += 2)
        .or_insert(1);

    println!("{:?}", fruits);

    println!("\n\n-----------------------\n\n");

    // HERE WE'RE GONNA MAKE A FN TO COUNT WORDS IN A TEXT

    let text = "this is a a long text with a lot of words that we will use to test the hashmap for counting words";

    let result = count_words(text);

    for (word, count) in result {
        println!("{}: {}", word, count);
    }

    println!("\n\n-----------------------\n\n");
}

fn count_words(text: &str) -> HashMap<&str, i32> {
    let mut words = HashMap::new();

    for word in text.split_whitespace() {
        words
            .entry(word)
            .and_modify(|value| *value += 1)
            .or_insert(1);
    }

    words
}
