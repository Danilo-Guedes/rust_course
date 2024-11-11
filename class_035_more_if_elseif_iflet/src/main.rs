fn main() {
    let value = 5;

    if value < 10 {
        println!("value is less than 10");
    } else if value == 10 {
        println!("value is 10");
    } else {
        println!("value is greater than 10");
    }

    let point = (4, 2);

    if let (0, 0) = point {
        // syntax is if let (pattern) = expression // if let statement is a shorthand for a match with just one pattern

        println!("point is zero")
    } else {
        println!("point is not zero")
    }

    // let's suppose we want to discover if the second value is within 1..4

    if let (_, y @ 1..=4) = point { // the @ operator allows us to check if the y is within the range 1..4
        println!("y is between 1 and 4 - the value is {y}");
    } else {
        println!("y is not between 1 and 4");
    }


    // if let .. else if let is also possible

    if let (0,0) = point {
        println!("point is at the origin");
    } else if let (_, y @ 1..=4) = point {
        println!("** y is between 1 and 4 - the value is {y}");
    } else {
        println!("point is not at the origin and y is not between 1 and 4");
    }
}
