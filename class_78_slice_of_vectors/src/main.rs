fn main() {
    println!("\n\n-----------------\n\n");

    let vec = vec![1, 2, 3, 4, 5];

    let slice = &vec[1..4];

    // the type of slice is &[i32]
    // this means that if you have a fn that accepts a slice of i32s, you can pass a slice of vectors or slice of arrays

    println!("{:?}", slice);

    println!("\n\n-----------------\n\n");
}
