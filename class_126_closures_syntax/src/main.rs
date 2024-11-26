fn main() {
    println!("Hello, world!");
}
// Closure without return statement and no body
let greet = || println!("Hello from closure!");

// Closure with multiple statements using a body
let add = |a: i32, b: i32| {
    let result = a + b;
    println!("Sum is {}", result);
    result
};

// Closure with more than one parameter
let multiply = |x: i32, y: i32| x * y;

// Using the closures
greet();
let sum = add(5, 3);
let product = multiply(4, 2);
println!("Product is {}", product);