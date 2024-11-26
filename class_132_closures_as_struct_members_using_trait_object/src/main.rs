// Define a struct with a member that stores a closure.
#[allow(dead_code)]
struct MyStruct {
    val: i32,
    // Closures don't have a fixed size at compile time because each closure can
    // capture variables in unique ways. Using `Box<dyn Fn(i32) -> i32>` allows us
    // to store the closure as a trait object, which has a fixed size (pointer and metadata).
    // could also be a &dyn Fn(i32) -> i32 using a reference to a trait object
    // or another smart pointer like "Rc" or "Arc" that implements the Defer trait.
    action: Box<dyn Fn(i32) -> i32>,
}

fn main() {
    println!("\n\n-----------------------\n\n");
    let y = 10;

    let closure = Box::new(move |x| x * y); // implement Fn trait because it does not mutate the environment

    let s = MyStruct { val: 10, action: closure };

    println!("{}", (s.action)(10)); // Output: 100

    println!("\n\n-----------------------\n\n");

}
