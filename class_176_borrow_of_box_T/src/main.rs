fn main() {
    let mut boxed_int = Box::new(10);

    // borrow Box as immutable raw pointer
    let immutable_raw_ptr = &*boxed_int as *const i32;

    // borrow Box as mutable raw pointer
    let mutable_raw_ptr = &mut *boxed_int as *mut i32;

    unsafe {
        *mutable_raw_ptr += 5;
        println!("Value via immutable raw pointer: {}", *immutable_raw_ptr);
    }
    unsafe {
        println!("Value via mutable raw pointer: {}", *mutable_raw_ptr);
    }
}
