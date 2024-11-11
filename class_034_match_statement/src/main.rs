fn main() {
    let x = 5;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => {
            // You can use curly braces to execute multiple statements
            println!("four");
        }
        5 => println!("five"),
        _ => println!("something else"),
    }

    let point = (3, -5);

    match point {
        (_, y) if y < 0 => {
            // x (you can ommit x bu using _) and y are bound to the values of the tuple, can be treated as local variables
            println!("second element is negative: {y}");
            println!("take action 1")
        }
        (0, 0) => {
            println!("Point is zero");
            println!("take action 2")
        }

        _ => println!("All fine"),
    }

    let array_to_check = [12, 42, 3, 94, -5];

    //check if an array has a negative number e set ot to invalid

    let is_array_invalid = match array_to_check { // when run cargo cluppy in this case it will show a warning sugesting using the matches! macro
        [n, ..] | [_, n, ..] | [_, _, n, ..] | [.., n] if n < 0 => true,
        _ => false,
    };
    println!("Is Array Invalid? =>  {}", is_array_invalid);

    // now lets try to use the matchs! macro

    let is_array_invalid_with_matches_macro = matches!(array_to_check, [n, ..] | [_, n, ..] | [_, _, n, ..] | [.., n] if n < 0);

    println!("Is Array Invalid with matches! macro? =>  {}", is_array_invalid_with_matches_macro);
}
