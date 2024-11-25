const MAX_VALUE: u32 = 100; // this cannot be mutable
const MESSAGE: &str = "Hello, world!";

static GLOBAL_VALUE: u32 = 200;
static mut MUT_GLOBAL_VALUE: u32 = 300; // this can be mutable but you need to use unsafe block to change it

fn main() {
    println!("\n\n-------------------\n\n");
    let ref_to_const = &MAX_VALUE; // the compiler replaces the reference with the actual value in compile time 
    println!("The maximum value is: {}", MAX_VALUE);
    println!("Message: {}", MESSAGE);
    println!("value: {}", ref_to_const);
    println!("\n\n-------------------\n\n");
    println!("Global value: {}", GLOBAL_VALUE);

    unsafe {
        println!("Global mutable value: {}", MUT_GLOBAL_VALUE);
        MUT_GLOBAL_VALUE = 400;
        println!("Global mutable value after change: {}", MUT_GLOBAL_VALUE);
    }
    println!("\n\n-------------------\n\n");
}
