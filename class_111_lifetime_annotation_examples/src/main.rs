fn find_smallest<'a>(a: &'a i32, b: &'a i32) -> &'a i32 { // here we're using a common lifetime annotation 'a
    if a < b {
        a
    } else {
        b
    }
}

fn main() {
    println!("\n\n----------------------\n\n");

    let a = 10;
    let b = 20;

    let result = find_smallest(&a, &b);

    println!("The smallest number is: {}", result);

    println!("\n\n----------------------\n\n");
}
