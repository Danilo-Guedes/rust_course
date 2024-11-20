fn find_biggest_item<'a>(strings: &'a [&'a str]) -> Option<&'a str> {
    let mut longest: Option<&str> = None;

    for i in strings {
        if longest.is_none() || i.len() > longest.unwrap().len() {
            longest = Some(i);
        }
    }
    longest
}

fn main() {
    println!("\n\n---------------------\n\n");

    let strings = ["cat", "dog", "elephant", "tiger"];

    let biggest_item = find_biggest_item(&strings);

    // println!("The biggest item is {:?}", biggest_item.unwrap()); // inthis way, if the return type is None, the program will panic

    match biggest_item {
        Some(item) => println!("The biggest item is {:?}", item),
        None => println!("There is no biggest item"),
    }

    println!("\n\n---------------------\n\n");
}
