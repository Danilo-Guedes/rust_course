fn main() {
    let value = 42; //i32
    let ref_of_value = &value; //&i32'
    println!("{}", *ref_of_value); // MANUAL DEREFERENCING  -> output: 42 -> *ref_of_value is dereferencing the reference
    println!("{}", ref_of_value); // AUTOMATIC DEREFERENCING  -> output: also 42 -> the compiler is smart enough to dereference the reference
    println!("{:p}", ref_of_value); // output: 0x7ffeeb1b3b7c -> memory address of value
    println!("{:p}", &ref_of_value); // output: 0x7ffeeb1b3b78 -> memory address of ref_of_value itself
}
