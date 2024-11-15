fn main() {
    // lets learnd the while loop

    let numbers = [88, 25, 66, 741];
    let mut index = 0;

    while index < numbers.len() {
        println!("in while index: {}", numbers[index]);

        index += 1;
    }

    println!("\n\n");

    // the while let loop is a more concise way to write a while loop that
    // uses pattern matching to destructure an Option or Result
    // very useful when the number of iterations is unknown and depends on the input

    let one_to_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut one_to_ten_iter = one_to_ten.iter();

    while let Some(n) = one_to_ten_iter.next() {
        if n % 2 == 0 {
            println!("even number: {}", n);
        } else {
            println!("odd number: {}", n);
        }
    }

    println!("\n\n");

    // loops as expression

    let other_numbers = [1, 2, 3, -4, 5];

    let message = 'outer: loop {
        for n in other_numbers {
            if n < 0 {
                break 'outer "found a negative number";
            }
        }
        break 'outer "all numbers are positive";
    };

    println!("message: {}", message);

    println!("\n\n");
}
