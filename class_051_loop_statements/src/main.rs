fn main() {
    // this are the different types of loops
    // 1 - loop
    // 2 - for in
    // 3 - while
    // 4 - while let

    let mut index = 0;
    loop {
        println!("index: {}", index);
        if index == 5 {
            break;
        }
        index += 1;
    }
    println!("simple loop done\n\n");

    // break with return value

    let mut index2 = 0;

    let value10 = loop {
        println!("index2: {}", index2);
        if index2 == 5 {
            break index2 * 2;
        }
        index2 += 1;
    };

    println!("loop with return value {}\n\n", value10);

    // we also can create loop with labels (even nested)
    // and they also can return values

    'outer: loop {
        println!("outer loop");
        'inner: loop {
            println!("inner loop 1");

            loop {
                println!("inner loop 2");
                break 'inner;
            }
        }
        break 'outer;
    }
    println!("Exited outer loop\n\n");

    // now let's see for in loop (cannot return values with break)

    'outerforin: for i in 0.. {
        println!("outerforin: {}", i);
        if i == 5 {
            println!("breaking for in loop\n\n");
            break 'outerforin; // in this case we cannot return a value
        }
    }

    for z in -5..=5 {
        // we can use negative values in the range operator
        println!("for in loop with negative: {}", z);
    }

    println!("\n\n");

    // we can use anything that implements the iterator trait

    let arr = [10, 20, 30, 40, 50];

    for element in arr.iter() {
        println!("for in loop with array: {}", element);
    }

    // $str.chars() returns an iterator

    let str = "Hello, world!";
    let target_letter = 'l';
    let mut count_letter = 0;

    for letter in str.chars() {
        if letter == target_letter {
            count_letter += 1;
        }
    }

    println!(
        "The letter {} appears {} times in the string '{}'\n\n",
        target_letter, count_letter, str
    );
}
