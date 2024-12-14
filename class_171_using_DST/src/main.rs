fn main() {
    println!("\n\n------------------------------------\n\n");

    let array = [1, 2, 3, 4, 5];

    // let slice = array[1..3];  error[E0277]: the size for values of type `[{integer}]` cannot be known at compilation time
    let slice = &array[1..3];

    println!("Slice: {:?}", slice);

    println!("\n\n------------------------------------\n\n");


    //USING BOX

    let array2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let s1 = Box::new(&array2[0..=2]);

    let s2 = array2[0..=2].to_vec().into_boxed_slice();

    println!("Boxed Slice: {:?}", s1);
    println!("Into Boxed Slice: {:?}", s2);


    println!("\n\n------------------------------------\n\n");






}
