fn main() {
    println!("\n\n---------------\n\n");

    // CAN WE COMPARE TWO VECTORS?

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![1, 2, 3];

    if vec1 < vec2 {
        // once the items inside the vectors implement the PartialOrd and PartialEqual trait, we can compare the vectors
        println!("vec1 is less than vec2");
    } else if vec1 > vec2 {
        println!("vec1 is greater than vec2");
    } else {
        println!("vec1 is equal to vec2");
    }

    println!("\n\n---------------\n\n");

    // PREPEND DATA USING INDEX

    let mut vec = vec![1, 2, 3];
    vec.insert(0, 0); // insert 0 at index 0 -> this can be computationally expensive if the vector is large because it requires shifting the elements

    println!("{:?}", vec); // [0, 1, 2, 3]

    println!("\n\n---------------\n\n");

    // PREPEND DATA USING SPLICE

    let mut vec = vec![1, 2, 3];
    vec.splice(0..0, vec![0]); // insert 0 at index 0 -> this is more efficient than using insert because it doesn't require shifting the elements

    println!("{:?}", vec); // [0, 1, 2, 3]

    println!("\n\n---------------\n\n");
}
