fn main() {
    let p = Point { x: 20, y: 20 };

    match p {
        // Point { x : x1, ..} if x1 == 20 => println!("x is 20"), // first more verbose implementation
        Point { x: 20, .. } => println!("x is 20"), // first more verbose implementation // notice that "x @ 20" does not work in this case
        Point { .. } => println!("x is not 20"),
    }

    println!("\n--------------\n");

    // is squared

    match p {
        Point { x, y } if x == y => println!("this is a square"),
        _ => println!("this is not a square"),
    }

    println!("\n--------------\n");

}

struct Point {
    x: i32,
    y: i32,
}
