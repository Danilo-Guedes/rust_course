fn main() {
    println!("\n\n---------------------\n\n");

    let mut v = vec![1, 2, 3, 4];

    // in the stack memory is created a pointer to the heap memory holding [capacity, Length, pointer] = [8, 4, 0x12345678]
    // in the heap memory is created an array of 4 elements [1, 2, 3, 4] with more 4 empty spaces

    println!("v = {:?}", v);

    println!("\n\n---------------------\n\n");


    // we can retrieve some metadata of this vector

    println!("v.len() = {}", v.len());

    println!("v.capacity() = {}", v.capacity());

    println!("\n\n---------------------\n\n");

    v.push(5); // this makes the vector to allocate more memory in the heap besides the 4 elements already there

    println!("after push 5 -> v.len() = {}", v.len());

    println!("after push 5 -> v.capacity() = {}", v.capacity());

    println!("\n\n---------------------\n\n");

    // we can create a vector with a specific capacity to avoid reallocations and gain performance

    let mut v2 = Vec::with_capacity(10);

    v2.push(1);
    v2.push(2);

    println!("v2.len() = {}", v2.len());

    println!("v2.capacity() = {}", v2.capacity());

    println!("\n\n---------------------\n\n");


}
