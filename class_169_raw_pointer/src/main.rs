fn main() {
    println!("\n\n-----------------\n\n");

    // *mut T is a raw pointer to a value of type T

    let mut x = 10;
    let raw_ptr = &mut x as *mut i32;
    unsafe {
        println!("raw_ptr: {:?}", *raw_ptr);
        *raw_ptr = 20;
        println!("raw_ptr: {:?}", *raw_ptr);
        println!("x: {:?}", x);

    }
    println!("\n\n-----------------\n\n");

    // *const T is a readonly raw pointer 

    let y = 10;

    let raw_ptr = &y as *const i32;

    unsafe {
        println!("raw_ptr: {:?}", *raw_ptr);
        // *raw_ptr = 50; THIS WILL ERROR
    }
}
