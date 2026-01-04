fn main() {
    let boxed_int = Box::new(100);

    // Convert Box into a mutable raw pointer
    // let raw_ptr : *mut i32 = Box::into_raw(boxed_int);

    // Convert Box into constant  raw pointer
    let raw_ptr: *const i32 = Box::into_raw(boxed_int) as *const i32;

    unsafe {
        // a safe way to deallocate memory
        let _ = Box::from_raw(raw_ptr as *mut i32);
    }

    // Conver Vec<i32> into a Box<[i32]>

    let vec_of_i32: Vec<i32> = vec![10, 100];
    // use Vec<T> when you need dinamic sizing

    let boxed_i32: Box<[i32]> = vec_of_i32.into_boxed_slice();
    // box slice makes the vector heap allocated and you cannot change its size anymore
    // use Box<[T]> when you need a heap allocated fixed size array

    println!("boxed_i32 = {:?}", boxed_i32);
}
