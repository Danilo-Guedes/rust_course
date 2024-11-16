/*
   tuples can be compared lexicographically if all of their constituent elements
   implement the 'PartialOrd' and 'PartialEq' traits.
*/
fn main() {
    let tuple1 = (2, 3, 4);
    let tuple2 = (8, 3, 4);
    let tuple3 = (1, 2, 3);

    if tuple1 < tuple2 {
        println!("tuple1 is smaller");
    } else {
        println!("tuple2 is smaller")
    }

    if tuple1 == tuple3 {
        println!("Tuples equal")
    } else {
        println!("Tuples not equal")
    }

    let tuple4 = (1, "world x");
    let tuple5 = (1, "world z");
    let tuple6 = (1, "world x");

    if tuple4 > tuple5 {
        println!("tuple4 is bigger")
    } else {
        println!("tuple5 is bigger")
    }

    if tuple4 == tuple6 {
        println!("Tuples equal")
    } else {
        println!("Tuples not equal")
    }

    println!("-----------------------------------");

    // using pattern matching

    let data = (1, "hello", 8);

    // let's log the second element only if the first element is greater than 0 and the las is smaller than 10

    match data {
        (a, b, c) if a > 0 && c < 10 => println!("Valid data: b = {}", b),
        _ => println!("Invalid data"),
    }

    println!("-----------------------------------");

    let find_soup_tuple = (1, "soup", 5);

    match find_soup_tuple {
        (_, "soup", _) => println!("Soup found"),
        _ => println!("Soup not found"),
    }

    println!("-----------------------------------");

    let tup = (1, 2, 20);

    // let's try to find if the third element in the tuple is between 10 and 20

    match tup {
        (_, _, c @ 10..=20) => println!("c is between 10 and 20: {}", c), // the @ is called a "binding operator"
        _ => println!("c is not between 10 and 20"),
    }

    println!("-----------------------------------");

    // we can use the rest operator to ignore the rest of the tuple

    let tup2 = (1, 2, 3, 4, 5, 6);

    match tup2 {
        (_, _, c, ..) if c > 2 => println!("c = {}", c),
        _ => println!("c is not greater than 2"),
    }


    println!("-----------------------------------");

    // using the binding operator to use the variable in other fn for example

    let tup3 = (1, 2, 3);

    match tup3 {
        (a @ 1, b @ 2, _) => {
            println!("this could be a fn call =>> a = {}, b = {}", a, b);
        }
        _ => println!("no match"),
    }
}
