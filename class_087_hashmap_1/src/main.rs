use std::collections::HashMap;

fn main() {
    println!("\n\n---------------\n\n");

    let mut fruit_prices = HashMap::new();

    fruit_prices.insert("apple", 0.50);
    fruit_prices.insert("banana", 0.25);
    fruit_prices.insert("orange", 0.30);

    let apple_price = fruit_prices.get("apple").unwrap_or(&0.0);

    println!("Apple costs: ${:.2}", apple_price);

    println!("\n\n---------------\n\n");

    for (fruit, price) in &fruit_prices {
        println!("{} costs: ${:.2}", fruit, price);
    }

    println!("\n\n---------------\n\n");

    // we also can do indexing

    let banana_price = fruit_prices["banana"];
    println!("Banana costs: ${:.2}", banana_price);

    println!("\n\n---------------\n\n");

    // keys

    for keys in fruit_prices.keys() {
        println!("Key: {}", keys);
    }

    println!("\n\n---------------\n\n");

    // values

    for values in fruit_prices.values() {
        println!("Value: ${:.2}", values);
    }

    println!("\n\n---------------\n\n");

    // ENTRY

    *fruit_prices.entry("apple").or_insert(0.0) += 1.0;
    *fruit_prices.entry("pear").or_insert(0.0) += 1.0;

    println!("{:?}", fruit_prices);

    println!("\n\n---------------\n\n");
}
