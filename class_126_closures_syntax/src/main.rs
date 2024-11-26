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

// closures can implement the traits of the Fn, FnMut, and FnOnce families depending on how they capture variables.

// Fn: The closure captures variables by reference.
// FnMut: The closure captures variables by mutable reference and mutate its enviroment.
// FnOnce: The closure captures variables by value and takes ownership of its enviroment, hence can be called only one time.

// the move keyword
// The move keyword can be used to force the closure to take ownership of the variables it captures depending ...
// the traits its arguments implement. This is useful when you want to move the variables into the closure's enviroment.